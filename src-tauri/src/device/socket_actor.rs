use std::{
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

use futures::{SinkExt, StreamExt};
use tauri::async_runtime::spawn;
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::error;

use super::{
    actor::{DeviceActorHandle, DeviceActorMessage},
    protocol::{ClientDeviceMessage, ServerDeviceMessage},
};

/// Connect to the websocket and return the stream
pub async fn websocket_connect(connect_url: &str) -> anyhow::Result<WsStream> {
    let (socket, _response) = tokio_tungstenite::connect_async(connect_url).await?;
    Ok(socket)
}

#[derive(Clone)]
pub struct DeviceSocketActorHandle {
    tx: mpsc::UnboundedSender<DeviceSocketActorMessage>,
}

impl DeviceSocketActorHandle {
    pub fn send_message(&self, msg: ClientDeviceMessage) -> anyhow::Result<()> {
        self.tx.send(DeviceSocketActorMessage::Message(msg))?;
        Ok(())
    }
}

pub enum DeviceSocketActorMessage {
    Message(ClientDeviceMessage),
    /// Tell the session to close
    Close,
}

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct DeviceSocketActor {
    /// Handle to send messages back to the device actor
    actor_tx: DeviceActorHandle,

    /// Underlying socket to read from and write to
    socket: WsStream,

    /// Receiver for messages to write to the socket
    rx: mpsc::UnboundedReceiver<DeviceSocketActorMessage>,

    /// State for writing to the socket
    write_state: WriteState,
}

impl DeviceSocketActor {
    pub fn start(socket: WsStream, actor_tx: DeviceActorHandle) -> DeviceSocketActorHandle {
        let (tx, rx) = mpsc::unbounded_channel();
        let tx = DeviceSocketActorHandle { tx };

        let actor = DeviceSocketActor {
            actor_tx,
            socket,
            rx,
            write_state: WriteState::Receive,
        };

        spawn(actor);

        tx
    }
}

enum WriteState {
    /// Waiting for an outbound message
    Receive,

    /// Write a message
    Write(Option<Message>),

    /// Flush a message
    Flush,
}

/// Possible outcomes from polling the read state
enum PollReadOutcome {
    /// Encountered an error
    Error(anyhow::Error),

    /// Encountered a no more messages state
    Closed,

    /// Encountered a continue state (Read ping/pong message)
    Continue,

    /// Read an actual device message
    Message(ServerDeviceMessage),
}

enum PollWriteOutcome {
    /// Encountered an error
    Error(anyhow::Error),

    /// Connection is closed
    Closed,

    /// Continue to next write
    Continue,
}

impl DeviceSocketActor {
    fn poll_read_message(&mut self, cx: &mut Context<'_>) -> Poll<PollReadOutcome> {
        let msg = match ready!(self.socket.poll_next_unpin(cx)) {
            Some(Ok(msg)) => msg,

            // Reading has errored
            Some(Err(err)) => return Poll::Ready(PollReadOutcome::Error(anyhow::Error::new(err))),

            // Socket has no more messages, socket has closed
            None => return Poll::Ready(PollReadOutcome::Closed),
        };

        let message = match msg {
            Message::Text(utf8_bytes) => utf8_bytes,

            // Ping and pong are handled internally
            Message::Ping(_) | Message::Pong(_) | Message::Frame(_) => {
                return Poll::Ready(PollReadOutcome::Continue)
            }

            // Expecting a text based protocol
            Message::Binary(_) => {
                return Poll::Ready(PollReadOutcome::Error(anyhow::anyhow!(
                    "unexpected binary message"
                )))
            }

            // Socket is closed
            Message::Close(_) => return Poll::Ready(PollReadOutcome::Closed),
        };

        let msg: ServerDeviceMessage = match serde_json::from_str(message.as_str()) {
            Ok(value) => value,

            // Failed to deserialize message
            Err(err) => return Poll::Ready(PollReadOutcome::Error(anyhow::Error::new(err))),
        };

        Poll::Ready(PollReadOutcome::Message(msg))
    }

    fn poll_write_message(&mut self, cx: &mut Context<'_>) -> Poll<PollWriteOutcome> {
        match &mut self.write_state {
            WriteState::Receive => {
                // Receive a message
                let result = match ready!(self.rx.poll_recv(cx)) {
                    Some(value) => value,
                    None => return Poll::Ready(PollWriteOutcome::Closed),
                };

                match result {
                    DeviceSocketActorMessage::Message(msg) => {
                        // Encode the message
                        let encoded = match serde_json::to_string(&msg) {
                            Ok(value) => value,
                            Err(err) => {
                                return Poll::Ready(PollWriteOutcome::Error(anyhow::Error::new(
                                    err,
                                )))
                            }
                        };

                        // Move to next state
                        let message = Message::text(encoded);
                        self.write_state = WriteState::Write(Some(message));
                    }
                    DeviceSocketActorMessage::Close => {
                        return Poll::Ready(PollWriteOutcome::Closed)
                    }
                }

                Poll::Ready(PollWriteOutcome::Continue)
            }
            WriteState::Write(message) => {
                if let Err(err) = ready!(self.socket.poll_ready_unpin(cx)) {
                    return Poll::Ready(PollWriteOutcome::Error(anyhow::Error::new(err)));
                }

                let packet = message
                    .take()
                    .expect("unexpected write state without a packet");

                if let Err(err) = self.socket.start_send_unpin(packet) {
                    return Poll::Ready(PollWriteOutcome::Error(anyhow::Error::new(err)));
                }

                self.write_state = WriteState::Flush;

                Poll::Ready(PollWriteOutcome::Continue)
            }
            WriteState::Flush => {
                if let Err(err) = ready!(self.socket.poll_flush_unpin(cx)) {
                    return Poll::Ready(PollWriteOutcome::Error(anyhow::Error::new(err)));
                }

                self.write_state = WriteState::Receive;

                Poll::Ready(PollWriteOutcome::Continue)
            }
        }
    }
}

impl Future for DeviceSocketActor {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        while let Poll::Ready(outcome) = this.poll_read_message(cx) {
            match outcome {
                // Continue to the next message
                PollReadOutcome::Continue => continue,

                PollReadOutcome::Error(cause) => {
                    error!(?cause, "device socket encountered error while reading");

                    // Stop the socket since we are in an error state
                    return Poll::Ready(());
                }
                PollReadOutcome::Message(message) => {
                    if this
                        .actor_tx
                        .send(DeviceActorMessage::ServerMessage(message))
                        .is_err()
                    {
                        // Channel to device actor is closed, device is closed
                        return Poll::Ready(());
                    }
                }

                // Socket has closed, finish the future
                PollReadOutcome::Closed => return Poll::Ready(()),
            }
        }

        while let Poll::Ready(outcome) = this.poll_write_message(cx) {
            match outcome {
                PollWriteOutcome::Continue => continue,
                PollWriteOutcome::Error(cause) => {
                    error!(?cause, "device socket encountered error while writing");

                    // Stop the socket since we are in an error state
                    return Poll::Ready(());
                }
                PollWriteOutcome::Closed => return Poll::Ready(()),
            }
        }

        Poll::Pending
    }
}

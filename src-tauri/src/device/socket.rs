use anyhow::Context;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, client::IntoClientRequest},
    MaybeTlsStream, WebSocketStream,
};
use tracing::warn;
use tungstenite::{
    error::ProtocolError as WebsocketProtocolError, Error as TWebsocketError,
    Message as WebsocketMessage,
};

use crate::events::{AppEvent, AppEventSender};

use super::protocol::{ClientDeviceMessage, ServerDeviceMessage};

pub struct DeviceClient {
    app_tx: AppEventSender,
    connect_url: String,
    device_name: String,
    access_token: Option<String>,
}

pub enum DeviceClientMessage {
    /// Send a message through the client
    Message(ClientDeviceMessage),

    Close,
}

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Connect to the websocket and return the stream
async fn websocket_connect(connect_url: &str) -> tungstenite::Result<WsStream> {
    tokio_tungstenite::connect_async(connect_url)
        .await
        // We only care about the socket
        .map(|(socket, _)| socket)
}

impl DeviceClient {
    pub fn new(
        app_tx: AppEventSender,
        connect_url: String,
        device_name: String,
        access_token: Option<String>,
    ) -> Self {
        Self {
            app_tx,
            connect_url,
            device_name,
            access_token,
        }
    }

    /// Run the websocket subscriber
    pub async fn run(self) -> anyhow::Result<()> {
        // Establish the stream
        let mut stream = websocket_connect(self.connect_url.as_str())
            .await
            .context("when establishing connection")?;

        _ = self.app_tx.send(AppEvent::DeviceConnected);

        if let Some(access_token) = self.access_token {
            _ = self.app_tx.send(AppEvent::DeviceAuthenticating);

            let msg = ClientDeviceMessage::Authenticate { access_token };
            let msg = serde_json::to_string(&msg)?;
            stream.send(WebsocketMessage::text(msg)).await?;
        } else {
            _ = self.app_tx.send(AppEvent::DeviceRequestingApproval);

            let msg = ClientDeviceMessage::RequestApproval {
                name: self.device_name.clone(),
            };
            let msg = serde_json::to_string(&msg)?;
            stream.send(WebsocketMessage::text(msg)).await?;
        }

        while let Some(msg) = stream.next().await {
            let msg = match msg {
                Ok(msg) => msg,
                // Can attempt reconnection from these errors
                Err(TWebsocketError::Protocol(
                    WebsocketProtocolError::ResetWithoutClosingHandshake,
                )) => {
                    warn!("connection lost, reestablishing it");
                    stream = websocket_connect(self.connect_url.as_str())
                        .await
                        .context("when reestablishing connection")?;
                    continue;
                }
                // Other errors can be considered fatal
                Err(err) => return Err(err.into()),
            };

            let msg = match msg {
                WebsocketMessage::Text(utf8_bytes) => utf8_bytes,

                _ => continue,
            };

            let msg: ServerDeviceMessage = serde_json::from_str(msg.as_str())?;
            match msg {
                ServerDeviceMessage::Approved {
                    device_id,
                    access_token,
                } => {
                    _ = self.app_tx.send(AppEvent::DeviceApproved {
                        device_id,
                        access_token,
                    });
                }
                ServerDeviceMessage::Authenticated {} => {
                    _ = self.app_tx.send(AppEvent::DeviceAuthenticating);
                }
                ServerDeviceMessage::ProfileUpdate {} => {
                    // TODO: Handle profile changes
                }
            }
        }

        Ok(())
    }
}

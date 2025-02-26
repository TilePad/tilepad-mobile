use anyhow::Context;
use futures::{stream::SplitSink, Sink, SinkExt, StreamExt};
use serde::Serialize;
use thiserror::Error;
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, client::IntoClientRequest},
    MaybeTlsStream, WebSocketStream,
};
use tracing::warn;
use tungstenite::{
    error::ProtocolError as TWsProtocolError, Error as TWsError, Message as WsMessage,
};

use crate::{
    config::{Controller, ControllerConnection, DeviceDetails},
    events::{AppEvent, AppEventSender},
};

use super::protocol::{AccessToken, ClientDeviceMessage, DeviceId, ServerDeviceMessage};

pub struct DeviceClientConnectionInfo {
    /// Details about the current device
    pub details: DeviceDetails,
    /// Controller we are connecting to
    pub controller: Controller,
}

pub struct DeviceClient {
    app_tx: AppEventSender,
    info: DeviceClientConnectionInfo,
}

pub enum DeviceClientMessage {
    /// Send a message through the client
    Message(ClientDeviceMessage),

    Close,
}

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug, Error)]
pub enum WsError {
    /// Error on the underlying protocol
    #[error(transparent)]
    Ws(#[from] TWsError),
    /// Error serializing or deserializing
    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    /// Request for approval was denied
    #[error("approval denied")]
    ApprovalDenied,

    #[error("invalid access token")]
    InvalidAccessToken,

    /// Got an unexpected message
    #[error("unexpected message")]
    UnexpectedMessage,

    /// Connection ended
    #[error("connection closed")]
    ConnectionClosed,
}

/// Connect to the websocket and return the stream
async fn websocket_connect(connect_url: &str) -> tungstenite::Result<WsStream> {
    tokio_tungstenite::connect_async(connect_url)
        .await
        // We only care about the socket
        .map(|(socket, _)| socket)
}

async fn write_message<M>(stream: &mut WsStream, msg: M) -> Result<(), WsError>
where
    M: Serialize,
{
    let msg = serde_json::to_string(&msg)?;
    stream.send(WsMessage::text(msg)).await?;
    Ok(())
}

async fn connect_websocket(client: &DeviceClient) -> Result<WsStream, WsError> {
    let (address, port) = match client.info.controller.connection {
        ControllerConnection::Socket { address, port } => (address, port),
    };

    let url = format!("ws://{}:{}/devices/ws", address, port);
    let stream = websocket_connect(&url).await?;
    Ok(stream)
}

/// Begin the connection process, authenticating or exchanging the user token
async fn start_authenticate(
    client: &mut DeviceClient,
    stream: &mut WsStream,
) -> Result<(), WsError> {
    match &client.info.controller.access_token {
        Some(access_token) => {
            match request_device_authenticate(client, stream, access_token.to_string()).await {
                Ok(_) => {}
                // Access token is no longer valid, attempt to request approval instead
                Err(WsError::InvalidAccessToken) => {
                    client.info.controller.access_token = None;
                    let access_token = request_device_approval(client, stream).await?;
                    client.info.controller.access_token = Some(access_token);
                }
                // Pass other errors upward
                Err(err) => return Err(err),
            }
        }
        None => {
            request_device_approval(client, stream).await?;
        }
    }

    // Report to the UI that we are authenticated
    _ = client.app_tx.send(AppEvent::ControllerAuthenticated {
        controller_id: client.info.controller.id,
    });

    Ok(())
}

async fn request_device_authenticate(
    client: &mut DeviceClient,
    stream: &mut WsStream,
    access_token: String,
) -> Result<(), WsError> {
    // Report to the UI that we are authenticating
    _ = client.app_tx.send(AppEvent::ControllerAuthenticating {
        controller_id: client.info.controller.id,
    });

    // Attempt authentication with the client
    write_message(stream, ClientDeviceMessage::Authenticate { access_token }).await?;

    // Wait for the approval response
    let msg = next_device_message(stream).await?;
    match msg {
        ServerDeviceMessage::InvalidAccessToken {} => Err(WsError::InvalidAccessToken),
        ServerDeviceMessage::Authenticated {} => Ok(()),

        _ => Err(WsError::UnexpectedMessage),
    }
}

async fn request_device_approval(
    client: &mut DeviceClient,
    stream: &mut WsStream,
) -> Result<AccessToken, WsError> {
    // Emit event that we are requesting approval
    _ = client.app_tx.send(AppEvent::RequestingControllerApproval {
        controller_id: client.info.controller.id,
    });

    // Write the approval request
    write_message(
        stream,
        ClientDeviceMessage::RequestApproval {
            name: client.info.details.device_name.clone(),
        },
    )
    .await?;

    // Wait for the approval response
    let msg = next_device_message(stream).await?;
    match msg {
        ServerDeviceMessage::Denied {} => {
            _ = client.app_tx.send(AppEvent::ControllerDenied {
                controller_id: client.info.controller.id,
            });
            Err(WsError::ApprovalDenied)
        }
        ServerDeviceMessage::Approved {
            device_id,
            access_token,
        } => {
            // Emit approval event
            _ = client.app_tx.send(AppEvent::ControllerApproved {
                controller_id: client.info.controller.id,
                device_id,
                access_token: access_token.clone(),
            });
            Ok(access_token)
        }
        _ => Err(WsError::UnexpectedMessage),
    }
}

/// Reads the next device related message from the socket (Ignoring ping/pong etc)
async fn next_device_message(stream: &mut WsStream) -> Result<ServerDeviceMessage, WsError> {
    loop {
        let msg = stream.next().await.ok_or(WsError::ConnectionClosed)??;
        let msg = match msg {
            WsMessage::Text(msg) => msg,
            _ => continue,
        };
        let msg: ServerDeviceMessage = serde_json::from_str(msg.as_str())?;
        return Ok(msg);
    }
}

pub async fn run_device() {

    // let stream =
    // let msg = match msg {
    //     Ok(msg) => msg,
    //     // Can attempt reconnection from these errors
    //     Err(WsError::Protocol(WsProtocolError::ResetWithoutClosingHandshake)) => {
    //         warn!("connection lost, reestablishing it");
    //         stream = websocket_connect(self.connect_url.as_str())
    //             .await
    //             .context("when reestablishing connection")?;
    //         continue;
    //     }
    //     // Other errors can be considered fatal
    //     Err(err) => return Err(err.into()),
    // };
}

impl DeviceClient {
    pub fn new(
        app_tx: AppEventSender,
        connect_url: String,
        device_name: String,
        access_token: Option<String>,
    ) -> Self {
        Self { app_tx }
    }
}

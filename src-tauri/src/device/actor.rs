use std::any;

use tauri::async_runtime::spawn;
use tokio::sync::mpsc;
use tracing::error;

use crate::{
    config::{Controller, ControllerConnection, ControllerId},
    events::{AppEvent, AppEventSender},
};

use super::{
    protocol::{AccessToken, ClientDeviceMessage, ServerDeviceMessage},
    socket_actor::{websocket_connect, DeviceSocketActor, DeviceSocketActorHandle},
};

pub struct DeviceActor {
    /// Handle to receive messages
    rx: mpsc::UnboundedReceiver<DeviceActorMessage>,

    /// ID of the controller this actor is connected to
    controller_id: ControllerId,

    /// Current state of the actor
    state: DeviceActorState,

    /// App event sender
    app_tx: AppEventSender,

    /// Handle to send messages to the socket
    socket_tx: Option<DeviceSocketActorHandle>,
}

#[derive(Default)]
pub enum DeviceActorState {
    #[default]
    NotAuthenticated,

    /// Device is authenticating
    Authenticating { device_name: String },

    /// Device is requesting approval
    RequestApproval { device_name: String },

    /// Device approval was declined
    NotApproved,

    /// Actively operating as a connected an authenticated socket
    Authenticated,
}

pub async fn connect_device(
    app_tx: AppEventSender,
    controller: &Controller,
) -> anyhow::Result<DeviceActorHandle> {
    let connect_url = match controller.connection {
        ControllerConnection::Socket { address, port } => {
            format!("ws://{address}:{port}/devices/ws")
        }
    };
    let socket = websocket_connect(&connect_url).await?;

    let device_actor = DeviceActor::start(controller.id, app_tx);
    let socket_actor = DeviceSocketActor::start(socket, device_actor.clone());
    device_actor.send(DeviceActorMessage::ConnectSocket {
        socket: socket_actor,
    })?;
    Ok(device_actor)
}

impl DeviceActor {
    pub fn start(controller_id: ControllerId, app_tx: AppEventSender) -> DeviceActorHandle {
        let (tx, rx) = mpsc::unbounded_channel();
        let tx = DeviceActorHandle { tx };

        let actor = DeviceActor {
            app_tx,
            controller_id,
            rx,
            socket_tx: None,
            state: Default::default(),
        };

        spawn(actor.run());

        tx
    }

    pub async fn run(mut self) {
        while let Some(msg) = self.rx.recv().await {
            match msg {
                DeviceActorMessage::ConnectSocket { socket } => {
                    self.socket_tx = Some(socket);
                }

                DeviceActorMessage::RequestInitiate {
                    device_name,
                    access_token,
                } => {
                    let result = if let Some(access_token) = access_token {
                        self.request_device_authenticate(device_name, access_token)
                    } else {
                        self.request_device_approval(device_name)
                    };

                    if let Err(cause) = result {
                        error!(?cause, "error requesting initiation for device actor");
                        break;
                    }
                }

                DeviceActorMessage::ServerMessage(msg) => {
                    if let Err(cause) = self.handle_message(msg) {
                        error!(?cause, "error handling message from device socket");
                        break;
                    }
                }

                // Close connection
                DeviceActorMessage::Close => break,
            };
        }
    }

    fn emit_event(&self, app_event: AppEvent) -> anyhow::Result<()> {
        // Emit event that we are requesting approval
        self.app_tx.send(app_event)?;
        Ok(())
    }

    pub fn request_device_approval(&mut self, device_name: String) -> anyhow::Result<()> {
        let socket_tx = match &self.socket_tx {
            Some(socket_tx) => socket_tx,
            None => return Err(anyhow::anyhow!("no device socket connected")),
        };

        // Emit event that we are requesting approval
        self.emit_event(AppEvent::RequestingControllerApproval {
            controller_id: self.controller_id,
        })?;

        // Send approval request
        socket_tx.send_message(ClientDeviceMessage::RequestApproval {
            name: device_name.clone(),
        })?;

        self.state = DeviceActorState::RequestApproval { device_name };

        Ok(())
    }

    pub fn request_device_authenticate(
        &mut self,
        device_name: String,
        access_token: String,
    ) -> anyhow::Result<()> {
        let socket_tx = match &self.socket_tx {
            Some(socket_tx) => socket_tx,
            None => return Err(anyhow::anyhow!("no device socket connected")),
        };

        // Emit event that we are requesting approval
        self.emit_event(AppEvent::ControllerAuthenticating {
            controller_id: self.controller_id,
        })?;

        // Send auth request
        socket_tx.send_message(ClientDeviceMessage::Authenticate { access_token })?;

        self.state = DeviceActorState::Authenticating { device_name };

        Ok(())
    }

    pub fn handle_message(&mut self, msg: ServerDeviceMessage) -> anyhow::Result<()> {
        match &self.state {
            // Should not accept any server messages in this state
            DeviceActorState::NotAuthenticated | DeviceActorState::NotApproved => {
                // TODO: err unexpected message
            }
            // State is authenticating
            DeviceActorState::Authenticating { device_name } => {
                match msg {
                    ServerDeviceMessage::Authenticated => {
                        self.state = DeviceActorState::Authenticated;
                    }
                    ServerDeviceMessage::InvalidAccessToken => {
                        // Access token was invalid, request a new token and approval
                        self.request_device_approval(device_name.clone())?;
                    }
                    _ => {
                        // TODO: err unexpected message
                    }
                }
            }
            DeviceActorState::RequestApproval { device_name } => {
                match msg {
                    ServerDeviceMessage::Denied => {
                        self.state = DeviceActorState::NotApproved;
                    }
                    ServerDeviceMessage::Approved {
                        device_id: _,
                        access_token,
                    } => {
                        // We have a token now, request authentication
                        self.request_device_authenticate(device_name.clone(), access_token)?;
                    }
                    _ => {
                        // TODO: err unexpected message
                    }
                }
            }
            DeviceActorState::Authenticated => {
                match msg {
                    ServerDeviceMessage::ProfileUpdate => {
                        // TODO: Handle profile changes
                    }

                    _ => {
                        // TODO: err unexpected message
                    }
                }
            }
        }

        Ok(())
    }
}

pub enum DeviceActorMessage {
    /// Provide a socket for the actor to use
    ConnectSocket {
        socket: DeviceSocketActorHandle,
    },

    RequestInitiate {
        device_name: String,
        access_token: Option<AccessToken>,
    },

    /// Message from the server
    ServerMessage(ServerDeviceMessage),

    // Stop the device
    Close,
}

#[derive(Clone)]
pub struct DeviceActorHandle {
    tx: mpsc::UnboundedSender<DeviceActorMessage>,
}

impl DeviceActorHandle {
    pub fn send(&self, msg: DeviceActorMessage) -> anyhow::Result<()> {
        self.tx.send(msg)?;
        Ok(())
    }
}

use tokio::sync::mpsc;
use uuid::Uuid;

use crate::config::ControllerId;

pub mod processing;

pub type AppEventSender = mpsc::UnboundedSender<AppEvent>;
pub type AppEventReceiver = mpsc::UnboundedReceiver<AppEvent>;

#[derive(Debug)]
pub enum AppEvent {
    // Device socket is connected
    ControllerConnected {
        controller_id: ControllerId,
    },
    // Device socket is disconnected
    ControllerDisconnected {
        controller_id: ControllerId,
    },

    // Device is authenticating
    ControllerAuthenticating {
        controller_id: ControllerId,
    },
    // Device is waiting for user approval on desktop end
    RequestingControllerApproval {
        controller_id: ControllerId,
    },

    /// Device was denied
    ControllerDenied {
        controller_id: ControllerId,
    },

    /// Device was approved and a token was granted
    ControllerApproved {
        controller_id: ControllerId,
        device_id: Uuid,
        access_token: String,
    },

    // Device is authenticated
    ControllerAuthenticated {
        controller_id: ControllerId,
    },
}

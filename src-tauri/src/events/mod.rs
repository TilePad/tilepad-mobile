use tokio::sync::mpsc;
use uuid::Uuid;

pub mod processing;

pub type AppEventSender = mpsc::UnboundedSender<AppEvent>;
pub type AppEventReceiver = mpsc::UnboundedReceiver<AppEvent>;

#[derive(Debug)]
pub enum AppEvent {
    // Device socket is connected
    DeviceConnected,
    // Device socket is disconnected
    DeviceDisconnected,

    // Device is authenticating
    DeviceAuthenticating,
    // Device is waiting for user approval on desktop end
    DeviceRequestingApproval,

    /// Device was approved and a token was granted
    DeviceApproved {
        device_id: Uuid,
        access_token: String,
    },

    // Device is authenticated
    DeviceAuthenticated,
}

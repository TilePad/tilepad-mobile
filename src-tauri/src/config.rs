use std::net::IpAddr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub details: DeviceDetails,
    pub controllers: Vec<Controller>,
}

/// Details about the current device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDetails {
    pub device_name: String,
}

pub type ControllerId = Uuid;

/// Control server that we are authorized to operate against
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Controller {
    /// Unique ID for the controller
    pub id: ControllerId,

    /// The connection details for the controller
    pub connection: ControllerConnection,

    /// Optional access token if we are authorized for communication
    pub access_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ControllerConnection {
    /// Connection through a socket
    Socket { address: IpAddr, port: u16 },
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Device message coming from the client side
#[derive(Serialize)]
#[serde(tag = "type")]
pub enum ClientDeviceMessage {
    /// Requests an access token and approval for a device
    RequestApproval {
        /// Name of the device
        name: String,
    },

    /// Get the current profile and all its tiles
    RequestProfile,

    /// Authenticate using a device access token
    Authenticate {
        /// Access token for making requests from a device
        access_token: String,
    },

    /// User has clicked a tile
    TileClicked {
        /// ID of the tile that was touches
        tile_id: Uuid,
    },
}

pub type DeviceId = Uuid;
pub type AccessToken = String;

/// Device message coming from the server side
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ServerDeviceMessage {
    /// Device access was denied
    Denied {},

    /// Device access was approved
    Approved {
        /// Unique ID of the device
        device_id: DeviceId,
        /// Device access token for future requests
        access_token: AccessToken,
    },

    /// Device is authenticated
    Authenticated {},

    /// Provided access token was invalid
    InvalidAccessToken {},

    /// Update the current profile data and its tiles
    ProfileUpdate {},
}

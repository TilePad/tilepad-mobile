use super::CmdResult;
use crate::database::{
    DbPool,
    entity::device::{CreateDevice, DeviceId, DeviceModel},
};
use anyhow::Context;
use tauri::State;

/// Get a list of devices
#[tauri::command]
pub async fn devices_get_devices(db: State<'_, DbPool>) -> CmdResult<Vec<DeviceModel>> {
    let devices = DeviceModel::all(db.inner()).await?;
    Ok(devices)
}

/// Create a new device
#[tauri::command]
pub async fn devices_create_device(
    db: State<'_, DbPool>,
    create: CreateDevice,
) -> CmdResult<DeviceModel> {
    let device = DeviceModel::create(db.inner(), create).await?;
    Ok(device)
}

/// Remove a device
#[tauri::command]
pub async fn devices_remove_device(db: State<'_, DbPool>, device_id: DeviceId) -> CmdResult<()> {
    DeviceModel::delete(db.inner(), device_id).await?;
    Ok(())
}

/// Update a specific device server public key
#[tauri::command]
pub async fn devices_set_server_public_key(
    db: State<'_, DbPool>,
    device_id: DeviceId,
    public_key: Option<Vec<u8>>,
) -> CmdResult<DeviceModel> {
    let device = DeviceModel::get_by_id(db.inner(), device_id)
        .await?
        .context("device not found")?;
    let device = device.set_server_public_key(db.inner(), public_key).await?;
    Ok(device)
}

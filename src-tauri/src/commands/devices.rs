use anyhow::Context;
use tauri::State;

use crate::database::{
    entity::device::{CreateDevice, DeviceId, DeviceModel, UpdateDevice},
    DbPool,
};

use super::CmdResult;

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

/// Update a specific device
#[tauri::command]
pub async fn devices_update_device(
    db: State<'_, DbPool>,
    device_id: DeviceId,
    update: UpdateDevice,
) -> CmdResult<DeviceModel> {
    let device = DeviceModel::get_by_id(db.inner(), device_id)
        .await?
        .context("device not found")?;
    let device = device.update(db.inner(), update).await?;
    Ok(device)
}

/// Get the name of the current mobile device
#[tauri::command]
pub async fn get_device_name() -> CmdResult<String> {
    let name = nick_name::NickName::new()?;
    Ok(name.get()?)
}

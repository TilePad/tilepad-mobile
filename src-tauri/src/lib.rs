use std::error::Error;

use anyhow::Context;
use tauri::{async_runtime::block_on, App, Manager};

mod commands;
mod config;
mod database;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use commands::devices;

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_keep_screen_on::init())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            devices::devices_get_devices,
            devices::devices_create_device,
            devices::devices_remove_device,
            devices::devices_update_device
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    #[cfg(mobile)]
    app.handle().plugin(tauri_plugin_barcode_scanner::init())?;

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .finish();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;

    let app_data_path = app
        .path()
        .app_data_dir()
        .context("failed to get app data dir")?;

    let db = block_on(database::connect_database(app_data_path.join("app.db")))
        .context("failed to load database")?;

    app.manage(db.clone());

    Ok(())
}

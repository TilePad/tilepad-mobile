use std::error::Error;
use tauri::{App, Manager, async_runtime::block_on};

mod commands;
mod database;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use commands::{devices, settings};

    tauri::Builder::default()
        .plugin(tauri_plugin_keep_screen_on::init())
        .plugin(tauri_plugin_haptics::init())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            // Devices
            devices::devices_get_devices,
            devices::devices_create_device,
            devices::devices_remove_device,
            devices::devices_set_server_public_key,
            // Settings
            settings::settings_get_settings,
            settings::settings_set_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    // Setup "bar code" scanning library for scanning QR codes
    #[cfg(mobile)]
    app.handle().plugin(tauri_plugin_barcode_scanner::init())?;

    // Setup tracing
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // Get the app path
    let app_data_path = app.path().app_data_dir()?;

    // Get the db file path
    let db_path = app_data_path.join("app.db");

    // Connect to the database
    let db = match block_on(database::connect_database(db_path)) {
        Ok(value) => value,
        Err(cause) => {
            tracing::error!(?cause, "failed to load database");
            std::process::exit(1);
        }
    };

    // Provide the database as app state
    app.manage(db);

    Ok(())
}

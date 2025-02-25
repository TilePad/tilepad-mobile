use std::error::Error;

use tauri::{async_runtime::spawn, App};
use tokio::sync::mpsc;

mod device;
mod events;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .finish();

    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;

    let app_handle = app.handle();

    let (app_event_tx, app_event_rx) = mpsc::unbounded_channel();

    // Spawn event processor
    spawn(events::processing::process_events(
        app_handle.clone(),
        app_event_rx,
    ));

    Ok(())
}

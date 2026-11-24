mod store;

use std::path::PathBuf;
use tauri::{AppHandle, Manager};

fn notes_path(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map(|dir| dir.join("notes.json"))
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn load_notes(app: AppHandle) -> Result<store::Notes, String> {
    store::load(&notes_path(&app)?).map_err(|error| error.to_string())
}

#[tauri::command]
fn save_note(app: AppHandle, title: String, body: String) -> Result<(), String> {
    store::save(&notes_path(&app)?, &title, &body).map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_notes, save_note])
        .run(tauri::generate_context!())
        .expect("failed to run bunbougu");
}

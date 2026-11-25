mod store;

use serde::Serialize;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

fn app_data(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map_err(|error| error.to_string())
}

#[derive(Serialize)]
struct Workspace {
    folder: PathBuf,
    notes: store::Notes,
}

#[tauri::command]
fn load_notes(app: AppHandle) -> (bool, Result<Workspace, String>) {
    let workspace = (|| {
        let folder = store::active_folder(&app_data(&app)?).map_err(|error| error.to_string())?;
        let notes = store::load(&folder).map_err(|error| error.to_string())?;
        Ok(Workspace { folder, notes })
    })();
    (cfg!(desktop), workspace)
}

#[cfg(desktop)]
#[tauri::command]
async fn choose_storage_folder(app: AppHandle) -> Result<Option<Workspace>, String> {
    use tauri_plugin_dialog::DialogExt;

    let Some(path) = app
        .dialog()
        .file()
        .set_title("保存フォルダーを選択")
        .blocking_pick_folder()
    else {
        return Ok(None);
    };
    let folder = path.into_path().map_err(|error| error.to_string())?;
    let notes = store::select_folder(&app_data(&app)?, &folder).map_err(|error| error.to_string())?;
    Ok(Some(Workspace { folder, notes }))
}

#[tauri::command]
fn save_note(app: AppHandle, title: String, body: String) -> Result<(), String> {
    let folder = store::active_folder(&app_data(&app)?).map_err(|error| error.to_string())?;
    store::save(&folder, &title, &body).map_err(|error| error.to_string())
}

#[tauri::command]
fn delete_note(app: AppHandle, title: String) -> Result<(), String> {
    let folder = store::active_folder(&app_data(&app)?).map_err(|error| error.to_string())?;
    store::delete(&folder, &title).map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();
    #[cfg(desktop)]
    let builder = builder
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            load_notes, save_note, delete_note, choose_storage_folder
        ]);
    #[cfg(mobile)]
    let builder = builder.invoke_handler(tauri::generate_handler![load_notes, save_note, delete_note]);

    builder
        .run(tauri::generate_context!())
        .expect("failed to run bunbougu");
}

mod presets;
mod gamedirs;       
mod jsonbuilder;
mod settings;

use crate::presets::*;
use crate::gamedirs::*;
use crate::jsonbuilder::*;
use crate::settings::*;

use tauri::{Manager, State};
use rusqlite::Connection;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use std::process::Command;


pub type TauriState = Mutex<AppState>;

pub const DB_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/state.db");

pub struct AppState {
    pub presets: HashMap<String, Preset>,
    pub game_dirs: Vec<GameDir>,
    pub dolphin_path: PathBuf,
}

pub fn save_dolph_path(path: &str) -> Result<(), String> {
    let conn = Connection::open(DB_PATH)
        .map_err(|e| format!("Failed to open database connection: {}", e))?;

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        rusqlite::params!["dolphin_path", path],
    )
    .map_err(|e| format!("Failed to save Dolphin path: {}", e))?;

    Ok(())
}

pub fn load_dolph_path() -> Result<PathBuf, String> {
    let conn = Connection::open(DB_PATH)
        .map_err(|e| format!("Failed to open database connection: {}", e))?;

    let mut stmt = conn
        .prepare("SELECT value FROM settings WHERE key = ?1")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let path: String = stmt
        .query_row(rusqlite::params!["dolphin_path"], |row| row.get(0))
        .map_err(|e| format!("Failed to load Dolphin path: {}", e))?;

    Ok(PathBuf::from(path))
}

#[tauri::command]
fn set_dolph_path(state: State<TauriState>, path: &str) {
    let mut app_state = state.lock().unwrap();
    app_state.dolphin_path = PathBuf::from(path);
    let _ = save_dolph_path(path).map_err(|e| {
        eprintln!("failed to save dolph path {e}")
    });
}


#[tauri::command]
fn get_dolph_path(state: State<TauriState>) -> PathBuf {
    let app_state = state.lock().unwrap();

    app_state.dolphin_path.clone()
}


#[tauri::command]
fn run_game(state: State<TauriState>, id: &str) -> Result<String, String> {
    let mut app_state = state.lock().unwrap();

    //will need to add error checking here
    app_state.dolphin_path = PathBuf::from("/Applications/Dolphin.app/Contents/MacOS/Dolphin");

    if let Some(preset) = app_state.presets.get_mut(id){
        let preset_name = preset.name.clone();
        let _ = create_json(&app_state, id)?;
        let mut json_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        json_path.push(format!("{}.json", preset_name));

        

        let child = Command::new(&app_state.dolphin_path)
            .arg("-e")
            .arg(json_path)
            .spawn()
            .map_err(|e| format!("failed to execute dolphin {}", e))?;

        

        return Ok(format!("started dolphin with pid: {}", child.id()));
        
    }
    else{
        return Err(String::from("preset to run not found"));
    }
    
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            initialise_db().expect("failed to create db");

            let presets = Preset::load_from_db().unwrap_or_else(|e| {
                eprintln!("Failed to load presets: {e}");
                HashMap::new() 
            });
            

            let game_dirs = GameDir::load_all_from_db().unwrap_or_else(|e| {
                eprintln!("failed to load game directories: {e}");
                Vec::new()
            });

            let dolphin_path = load_dolph_path().unwrap_or_else(|e| {
                eprintln!("failed to load dolphin path {e}");
                PathBuf::new()
            });

            
            
            let state = AppState {
                presets,
                game_dirs,
                dolphin_path
            };

            app.manage(Mutex::new(state));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_preset, get_presets, 
            set_selection, create_gamedir, get_gamedirs, 
            remove_gamedir, set_game_path, get_path_name, run_game,
            get_dolph_path, set_dolph_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

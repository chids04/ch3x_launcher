mod presets;
mod gamedirs;       
mod jsonbuilder;

use crate::presets::*;
use crate::gamedirs::*;
use crate::jsonbuilder::*;

use tauri::{Manager, State};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use std::process::Command;
use std::fs;


pub type TauriState = Mutex<AppState>;

pub const DATA_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/app_data.json");

#[derive(Serialize, Deserialize, Clone)]
pub struct AppData {
    pub presets: HashMap<String, Preset>,
    pub game_dirs: Vec<GameDir>,
    pub dolphin_path: PathBuf,
}

impl AppData {
    pub fn load_or_default() -> Self {
        let data_path = PathBuf::from(DATA_PATH);
        
        if data_path.exists() {
            match fs::read_to_string(&data_path) {
                Ok(content) => {
                    match serde_json::from_str::<AppData>(&content) {
                        Ok(data) => return data,
                        Err(e) => eprintln!("Failed to parse app data: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to read app data: {}", e),
            }
        }
        
        Self::default()
    }
    
    pub fn save(&self) -> Result<(), String> {
        let data_path = PathBuf::from(DATA_PATH);
        
        if let Some(parent) = data_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create data directory: {}", e))?;
        }
        
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize app data: {}", e))?;
        
        fs::write(&data_path, json)
            .map_err(|e| format!("Failed to write app data: {}", e))?;
        
        Ok(())
    }
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            presets: HashMap::new(),
            game_dirs: Vec::new(),
            dolphin_path: PathBuf::new(),
        }
    }
}

pub struct AppState {
    pub data: AppData,
    pub dirty: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            data: AppData::load_or_default(),
            dirty: false,
        }
    }
    
    pub fn save_if_dirty(&mut self) -> Result<(), String> {
        if self.dirty {
            self.data.save()?;
            self.dirty = false;
        }
        Ok(())
    }
    
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }
}

#[tauri::command]
fn set_dolph_path(state: State<TauriState>, path: &str) -> Result<(), String> {
    let mut app_state = state.lock().unwrap();
    app_state.data.dolphin_path = PathBuf::from(path);
    app_state.mark_dirty();
    app_state.save_if_dirty()
}


#[tauri::command]
fn get_dolph_path(state: State<TauriState>) -> PathBuf {
    let app_state = state.lock().unwrap();
    app_state.data.dolphin_path.clone()
}


#[tauri::command]
fn run_game(state: State<TauriState>, id: &str) -> Result<String, String> {
    let mut app_state = state.lock().unwrap();

    if let Some(preset) = app_state.data.presets.get_mut(id){
        let preset_name = preset.name.clone();
        let _ = create_json(&app_state.data, id)?;
        let mut json_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        json_path.push(format!("{}.json", preset_name));

        let child = Command::new(&app_state.data.dolphin_path)
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
            let state = AppState::new();
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

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use std::fs;

use crate::gamedirs::GameDir;
use crate::presets::Preset;

#[derive(Serialize, Deserialize, Clone)]
pub struct AppData {
    pub presets: HashMap<String, Preset>,
    pub game_dirs: Vec<GameDir>,
    pub dolphin_path: PathBuf,
}

pub type TauriState = Mutex<AppState>;

pub const DATA_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/app_data.json");

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
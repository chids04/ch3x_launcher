mod presets;
mod gamedirs;       
mod jsonbuilder;
mod state;

use crate::presets::*;
use crate::gamedirs::*;
use crate::jsonbuilder::*;
use crate::state::*;

use tauri::{Manager, State};

use std::path::PathBuf;
use std::sync::Mutex;
use std::process::Command;


#[tauri::command]
fn set_dolph_path(state: State<TauriState>, path: &str) -> Result<(), String> {
    let mut app_state = state.lock().unwrap();
    app_state.data.dolphin_path = PathBuf::from(path);
    app_state.mark_dirty();
    app_state.save_if_dirty()
}

#[tauri::command]
fn remove_preset(state: State<TauriState>, id: &str) -> Result<(), String> {
    dbg!("deleting preset");

    let mut app_state = state.lock().unwrap();

    if app_state.data.presets.contains_key(id) {
        app_state.data.presets.remove(id);

        if let Err(e) = app_state.data.save() {
            return Err(e);
        }

        Ok(())
    }
    else {
        return Err("Failed to remove preset, preset already deleted".into());
    }
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
            get_dolph_path, set_dolph_path, remove_preset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

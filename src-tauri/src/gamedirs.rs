
use tauri::State;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

use crate::TauriState;

#[derive(Clone, Serialize, Deserialize)]
pub struct GameDir {
    pub name: String,
    pub path: PathBuf,
}

//allows for quick selection of the base game for the mods to be applied to
#[tauri::command]
pub fn create_gamedir(state: State<TauriState>, name: &str, path: &str) -> Result<(), String> {
    let mut app_state = state.lock().unwrap();

    if app_state.data.game_dirs.iter().any(|g| g.path == PathBuf::from(path)) {
        return Err(String::from("path already exists"));
    }

    if app_state.data.game_dirs.iter().any(|g| g.name == name.to_string()) {
        return Err(String::from("name already exists"));
    }

    let game_dir = GameDir {
        name: name.to_string(),
        path: PathBuf::from(path)
    };

    app_state.data.game_dirs.push(game_dir);
    app_state.mark_dirty();
    app_state.save_if_dirty()
}


#[tauri::command]
pub fn get_gamedirs(state: State<TauriState>) -> Vec<GameDir> {
    let app_state = state.lock().unwrap();
    app_state.data.game_dirs.clone()
}

#[tauri::command]
pub fn remove_gamedir(state: State<TauriState>, index: usize) {
    let mut app_state = state.lock().unwrap();

    if index < app_state.data.game_dirs.len() {
        app_state.data.game_dirs.remove(index);
        app_state.mark_dirty();
        let _ = app_state.save_if_dirty();
    } else {
        eprintln!("remove_gamedir: index out of range");
    }
}

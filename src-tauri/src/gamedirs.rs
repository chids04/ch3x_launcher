
use tauri::{ Manager, State };
use std::collections::HashMap;
use std::sync::Mutex;
use std::path::PathBuf;
use serde::Serialize;
use rusqlite::Connection;

use crate::TauriState;
use crate::DB_PATH;

#[derive(Clone, Serialize)]
pub struct GameDir {
    pub name: String,
    pub path: PathBuf,
}

impl GameDir  {
    pub fn save_to_db(&self) -> Result<(), String> {
        let conn = Connection::open(DB_PATH)
            .map_err(|e| format!("Failed to open database connection: {}", e))?;
        conn.execute(
            "INSERT OR REPLACE INTO game_dirs (name, path) VALUES (?1, ?2)",
            rusqlite::params![self.name, self.path.to_string_lossy()],
        )
        .map_err(|e| format!("Failed to save game directory: {}", e))?;

        Ok(())
    }

    pub fn load_all_from_db() -> Result<Vec<Self>, String> {
        let conn = Connection::open(DB_PATH)
            .map_err(|e| format!("Failed to open database connection: {}", e))?;
        let mut stmt = conn
            .prepare("SELECT name, path FROM game_dirs")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let game_dirs = stmt
            .query_map([], |row| {
                Ok(GameDir {
                    name: row.get(0)?,
                    path: PathBuf::from(row.get::<_, String>(1)?),
                })
            })
            .map_err(|e| format!("Failed to query game directories: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect game directories: {}", e))?;

        Ok(game_dirs)
    }
}

#[tauri::command]
pub fn create_gamedir(state: State<TauriState>, name: &str, path: &str) -> Result<(), String> {
    let mut app_state = state.lock().unwrap();

    if app_state.game_dirs.iter().any(|g| g.path == PathBuf::from(path)) {
        return Err(String::from("path already exists"));
    }

    // Check if the name already exists
    if app_state.game_dirs.iter().any(|g| g.name == name.to_string()) {
        return Err(String::from("name already exists"));
    }

    let game_dir = GameDir {
        name: name.to_string(),
        path: PathBuf::from(path)
    };

    game_dir.save_to_db()?;
    app_state.game_dirs.push(game_dir);
    Ok(())
}


#[tauri::command]
pub fn get_gamedirs(state: State<TauriState>) -> Vec<GameDir> {
    let app_state = state.lock().unwrap();
    return app_state.game_dirs.clone()
}

#[tauri::command]
pub fn remove_gamedir(state: State<TauriState>, index: usize) {
    let mut app_state = state.lock().unwrap();

    if index < app_state.game_dirs.len() {
        let game_dir_to_remove = &app_state.game_dirs[index];

        let conn = Connection::open(DB_PATH).map_err(|e| format!("Failed to open database connection: {}", e)).unwrap();
        conn.execute(
            "DELETE FROM game_dirs WHERE name = ?1 AND path = ?2",
            rusqlite::params![game_dir_to_remove.name, game_dir_to_remove.path.to_string_lossy()],
        ).map_err(|e| format!("Failed to delete game directory: {}", e)).unwrap();

        app_state.game_dirs.remove(index);

        } else {
        eprintln!("remove_gamedir: index out of range");
    }
}

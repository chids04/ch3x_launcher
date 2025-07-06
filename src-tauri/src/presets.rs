use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::collections::HashMap;

use tauri::State;
use serde::Serialize;
use time::UtcDateTime;
use xml::reader::{EventReader, XmlEvent};
use rusqlite::{Connection, params};

use crate::TauriState;
use crate::DB_PATH;

#[derive(Serialize, Clone)]
pub struct Preset {
    pub id: String,
    pub name: String,
    pub options: Vec<PresetOpt>,
    pub created_at: UtcDateTime,
    pub game_path: PathBuf,
    pub xml_path: PathBuf,
    pub section_name: String,
}

#[derive(Serialize, Clone, Default)]
pub struct PresetOpt {
    pub name: String,
    pub selected: String,
    pub choices: Vec<String>,
}

impl Preset {
    pub fn save_to_db(&self) -> Result<(), String> {
        let conn = Connection::open(DB_PATH)
            .map_err(|e| format!("Failed to open database connection: {}", e))?;
        
        let created_at_json = serde_json::to_string(&self.created_at)
            .map_err(|e| format!("Failed to serialize date: {}", e))?;
        
        conn.execute(
            "INSERT OR REPLACE INTO presets (id, name, game_path, xml_path, section_name, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                self.id,
                self.name,
                self.game_path.to_string_lossy(),
                self.xml_path.to_string_lossy(),
                self.section_name,
                created_at_json, // Store JSON string
            ],
        )
        .map_err(|e| format!("Failed to save preset: {}", e))?;

        for opt in &self.options {
            opt.save_to_db(&self.id)?;
        }

        Ok(())
    }

    pub fn load_from_db() -> Result<HashMap<String, Self>, String> {
        let preset_opts_map = Self::load_all_preset_options()?;
        
        let mut presets_map = Self::load_all_presets()?;
        
        Self::attach_options_to_presets(&mut presets_map, preset_opts_map);
        
        Ok(presets_map)
    }
    
    fn load_all_preset_options() -> Result<HashMap<String, Vec<PresetOpt>>, String> {
        let conn = Connection::open(DB_PATH)
            .map_err(|e| format!("Failed to open database connection for options: {}", e))?;
        
        let mut stmt = conn
            .prepare("SELECT preset_id, name, selected, choices FROM preset_opts")
            .map_err(|e| format!("Failed to prepare preset_opts statement: {}", e))?;
        
        let mut preset_opts_map: HashMap<String, Vec<PresetOpt>> = HashMap::new();
        
        let opts_iter = stmt.query_map([], |row| {
            let preset_id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let selected: String = row.get(2)?;
            let choices_json: String = row.get(3)?;
            
            let choices: Vec<String> = serde_json::from_str(&choices_json)
                .map_err(|e| {
                    eprintln!("Failed to parse choices JSON: {}", e);
                    rusqlite::Error::InvalidQuery
                })?;
            
            Ok((preset_id, PresetOpt {
                name,
                selected,
                choices,
            }))
        }).map_err(|e| format!("Failed to query preset options: {}", e))?;
        
        for opt_result in opts_iter {
            let (preset_id, opt) = opt_result
                .map_err(|e| format!("Failed to process option: {}", e))?;
            preset_opts_map
                .entry(preset_id)
                .or_insert_with(Vec::new)
                .push(opt);
        }
        
        Ok(preset_opts_map)
    }
    
    fn load_all_presets() -> Result<HashMap<String, Self>, String> {
        let conn = Connection::open(DB_PATH)
            .map_err(|e| format!("Failed to open database connection for presets: {}", e))?;
        
        let mut stmt = conn
            .prepare("SELECT id, name, game_path, xml_path, section_name, created_at FROM presets")
            .map_err(|e| format!("Failed to prepare presets statement: {}", e))?;
        
        let mut presets_map = HashMap::new();
        
        let presets_iter = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let game_path_str: String = row.get(2)?;
            let xml_path_str: String = row.get(3)?;
            let section_name: String = row.get(4)?;
            let created_at_json: String = row.get(5)?;
            
            let created_at = serde_json::from_str::<UtcDateTime>(&created_at_json)
                .map_err(|e| {
                    eprintln!("Failed to parse date JSON: {}", e);
                    rusqlite::Error::InvalidQuery
                })?;
            
            Ok((id.clone(), Preset {
                id,
                name,
                game_path: PathBuf::from(game_path_str),
                xml_path: PathBuf::from(xml_path_str),
                section_name,
                created_at,
                options: Vec::new(), // Start with empty options
            }))
        }).map_err(|e| format!("Failed to query presets: {}", e))?;
        
        for preset_result in presets_iter {
            let (id, preset) = preset_result
                .map_err(|e| format!("Failed to process preset: {}", e))?;
            presets_map.insert(id, preset);
        }
        
        Ok(presets_map)
    }
    
    fn attach_options_to_presets(
        presets_map: &mut HashMap<String, Self>, 
        mut preset_opts_map: HashMap<String, Vec<PresetOpt>>
    ) {
        for (preset_id, preset) in presets_map.iter_mut() {
            if let Some(options) = preset_opts_map.remove(preset_id) {
                preset.options = options;
            }
        }
    }
}

impl PresetOpt {
    fn new() -> Self {
        PresetOpt {
            name: String::new(),
            selected: String::new(),
            choices: Vec::new(),
        }
    }

    pub fn save_to_db(&self, preset_id: &str) -> Result<(), String> {
        let conn = Connection::open(DB_PATH)
            .map_err(|e| format!("Failed to open database connection: {}", e))?;

        let choices_json = serde_json::to_string(&self.choices)
            .map_err(|e| format!("Failed to serialize choices: {}", e))?;

        conn.execute(
            "INSERT OR REPLACE INTO preset_opts (preset_id, name, selected, choices) VALUES (?1, ?2, ?3, ?4)",
            params![preset_id, self.name, self.selected, choices_json],
        )
        .map_err(|e| format!("Failed to save preset option: {}", e))?;

        Ok(())
    }

    pub fn load_all_from_db(preset_id: &str) -> Result<Vec<Self>, String> {
        let conn = Connection::open(DB_PATH).map_err(|e| e.to_string())?;
        Self::load_all_from_db_with_conn(&conn, preset_id)
    }

    pub fn load_all_from_db_with_conn(conn: &Connection, preset_id: &str) -> Result<Vec<Self>, String> {
        let mut stmt = conn
            .prepare("SELECT name, selected, choices FROM preset_opts WHERE preset_id = ?1")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let opts = stmt
            .query_map(params![preset_id], |row| {
                let choices_json: String = row.get(2)?;
                let choices: Vec<String> = serde_json::from_str(&choices_json)
                    .map_err(|_| rusqlite::Error::InvalidQuery)?;

                Ok(PresetOpt {
                    name: row.get(0)?,
                    selected: row.get(1)?,
                    choices,
                })
            })
            .map_err(|e| format!("Failed to load preset options: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect preset options: {}", e))?;

        Ok(opts)
    }
}

impl fmt::Display for Preset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Preset:\n  ID: {}\n  Name: {}\n  Created At: {}\n  Options:\n{}",
            self.id,
            self.name,
            self.created_at,
            self.options
                .iter()
                .map(|opt| format!("    {}", opt))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl fmt::Display for PresetOpt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Option:\n      Name: {}\n      Selected: {}\n      Choices: [{}]",
            self.name,
            self.selected,
            self.choices.join(", ")
        )
    }
}

#[tauri::command]
pub fn create_preset(state: State<TauriState>, id: &str, name: &str, xml_path: &str) -> Result<(), String> {
    let file = File::open(xml_path).map_err(|_| String::from("Failed to open XML file"))?;
    let file = BufReader::new(file); 
    let parser = EventReader::new(file);

    let mut current_opt: Option<&mut PresetOpt> = None;
    let mut options: Vec<PresetOpt> = Vec::new();
    let mut section_name = String::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.local_name == "option" {
                    let mut opt = PresetOpt::new();
                    if let Some(attr) = attributes.iter().find(|attr| attr.name.local_name == "name") {
                        opt.name = attr.value.clone();
                    }
                    opt.choices.push("Disabled".to_string());
                    options.push(opt);
                    current_opt = options.last_mut();
                } else if name.local_name == "choice" {
                    if let Some(opt) = current_opt.as_mut() {
                        if let Some(attr) = attributes.iter().find(|attr| attr.name.local_name == "name"){
                            opt.choices.push(attr.value.clone());
                        }
                    }
                } else if name.local_name == "section" {
                    if let Some(attr) = attributes.iter().find(|attr| attr.name.local_name == "name") {
                        section_name = attr.value.clone();
                    }
                }
            }

            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "options" {
                    break
                } else if name.local_name == "option" {
                    if let Some(opt) = current_opt.as_mut() {
                        if !opt.choices.is_empty() {
                            opt.selected = opt.choices[0].clone();
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            _ => {}
        }
    }

    if options.is_empty() {
        return Err(String::from("No options found in XML file"));
    }

    let preset = Preset {
        id: id.to_string(),
        name: name.to_string(),
        options,
        created_at: UtcDateTime::now(),
        game_path: PathBuf::new(),
        xml_path: PathBuf::from(xml_path),
        section_name,
    };

    let mut app_state = state.lock().unwrap();
    preset.save_to_db()?;
    app_state.presets.insert(id.to_string(), preset);

    Ok(())
}

#[tauri::command]
pub fn get_presets(state: State<TauriState>) -> Vec<Preset> {
    let app_state = state.lock().unwrap();
    let mut presets: Vec<Preset> = app_state.presets.values().cloned().collect();
    presets.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    presets
}

#[tauri::command]
pub fn set_selection(state: State<TauriState>, id: &str, name: &str, selection: &str) -> Result<(), String> {
    let mut app_state = state.lock().unwrap();

    let preset = app_state.presets.get_mut(id).ok_or("Preset not found".to_string())?;
    let option = preset.options.iter_mut().find(|opt| opt.name == name).ok_or("Option not found".to_string())?;
    option.selected = selection.to_string();
    option.save_to_db(&preset.id)?;

    Ok(())
}

#[tauri::command]
pub fn set_game_path(state: State<TauriState>, id: &str, path: &str) {
    let mut app_state = state.lock().unwrap();
    if let Some(preset) = app_state.presets.get_mut(id) {
        preset.game_path = PathBuf::from(path);
        preset.save_to_db().unwrap_or_else(|e| eprintln!("failed saving game dir to file {e}"));
        println!("new game path: {:?}", preset.game_path);
    }
}

#[tauri::command]
pub fn get_path_name(state: State<TauriState>, id: &str) -> Option<String> {
    let app_state = state.lock().unwrap();
    if let Some(preset) = app_state.presets.get(id) {
        if let Some(dir) = app_state.game_dirs.iter().find(|g| g.path == preset.game_path) {
            return Some(dir.name.clone());
        }
    }
    None
}

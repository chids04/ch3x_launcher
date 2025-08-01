use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use tauri::State;
use serde::{Serialize, Deserialize};
use time::UtcDateTime;
use xml::reader::{EventReader, XmlEvent};

use crate::TauriState;

#[derive(Serialize, Deserialize, Clone)]
pub struct Preset {
    pub id: String,
    pub name: String,
    pub options: Vec<PresetOpt>,
    pub created_at: UtcDateTime,
    pub game_path: PathBuf,
    pub xml_path: PathBuf,
    pub section_name: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PresetOpt {
    pub name: String,
    pub selected: String,
    pub choices: Vec<String>,
}

impl Preset {
    // No longer need save_to_db or load_from_db methods
    // All persistence is handled by AppData
}

impl PresetOpt {
    fn new() -> Self {
        PresetOpt {
            name: String::new(),
            selected: String::new(),
            choices: Vec::new(),
        }
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

//creates a game preset
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
                    
                    //all presets need an extra 'disabled' field
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
    app_state.data.presets.insert(id.to_string(), preset);
    app_state.mark_dirty();
    app_state.save_if_dirty()?;

    Ok(())
}

#[tauri::command]
pub fn get_presets(state: State<TauriState>) -> Vec<Preset> {
    let app_state = state.lock().unwrap();
    let mut presets: Vec<Preset> = app_state.data.presets.values().cloned().collect();
    presets.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    presets
}

#[tauri::command]
pub fn set_selection(state: State<TauriState>, id: &str, name: &str, selection: &str) -> Result<(), String> {
    let mut app_state = state.lock().unwrap();

    let preset = app_state.data.presets.get_mut(id).ok_or("Preset not found".to_string())?;
    let option = preset.options.iter_mut().find(|opt| opt.name == name).ok_or("Option not found".to_string())?;    option.selected = selection.to_string();
    app_state.mark_dirty();
    app_state.save_if_dirty()
}

#[tauri::command]
pub fn set_game_path(state: State<TauriState>, id: &str, path: &str) {
    let mut app_state = state.lock().unwrap();
    if let Some(preset) = app_state.data.presets.get_mut(id) {
        preset.game_path = PathBuf::from(path);
        println!("new game path: {:?}", preset.game_path);
    }
    app_state.mark_dirty();
    app_state.save_if_dirty().unwrap_or_else(|e| eprintln!("failed saving state to file {e}"));
}

#[tauri::command]
pub fn get_path_name(state: State<TauriState>, id: &str) -> Option<String> {
    let app_state = state.lock().unwrap();
    if let Some(preset) = app_state.data.presets.get(id) {
        if let Some(dir) = app_state.data.game_dirs.iter().find(|g| g.path == preset.game_path) {
            return Some(dir.name.clone());
        }
    }
    None
}

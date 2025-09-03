use serde_json::json;
use std::path::{PathBuf, Path};

use crate::AppData;

#[cfg(test)]
use std::sync::Mutex;

#[cfg(test)]  
use crate::TauriState;

//creates a json file that can be read by Dolphin
pub fn create_json(app_data: &AppData, id: &str) -> Result<(), String>{
    let preset = match app_data.presets.get(id){
        Some(preset) => preset,
        None => return Err(String::from("preset not found, delete it")),
    };

    if preset.game_path == PathBuf::new(){
        return Err(String::from("missing game path, please select one"));
    }


    //grouping the presetopts together
    let opts: Result<Vec<_>, String> = preset.options
        .iter()
        .map(|o| {
            if o.selected == String::new(){
                return Err(format!("missing choice {}",o.name))
            }
            else{
                if let Some(idx) = o.choices.iter().position(|choice| *choice == o.selected){
                    return Ok(json!({
                        "choice" : idx,
                        "option-name" : o.name,
                        "section-name" : preset.section_name
                    }))
                }
                else{
                    return Err(format!("invalid choice for {}", o.name));
                }

            }
        })
        .collect();

    let opts = opts?;

    let dir_name = preset.xml_path.parent().ok_or_else(|| {
        String::from("xml path has no parent directory")
    })?;

    if dir_name.file_name().unwrap() != "riivolution"{
        return Err(String::from("xml must be in riivolution folder"));
    }

    let root_dir = dir_name.parent().ok_or_else(|| {
        String::from("riivolution folder must have a parent")
    })?;

    let json_file = json!({
        "base-file" : preset.game_path,
        "display-name" : preset.name,
        "riivolution" : {
            "patches" : [ 
                {
                    "options" : opts,
                    "root" : root_dir,
                    "xml" : preset.xml_path
                }
            ]
        },
        "type" : "dolphin-game-mod-descriptor",
        "version" : 1
    });

    let cargo_toml_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let output_path = cargo_toml_path.join(format!("{}.json", preset.name));

    std::fs::write(&output_path, serde_json::to_string_pretty(&json_file).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;

    Ok(())
    
    
}


#[cfg(test)]
mod tests {
    use super::*;
    use tauri::Manager;
    use time::UtcDateTime;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use crate::presets::{Preset, PresetOpt};
    use crate::AppState;

    #[test]
    pub fn test_create_json() {
        let app = tauri::test::mock_app();

        let mut mock_state = AppState{
            data: crate::AppData {
                game_dirs: Vec::new(),
                presets: HashMap::new(),
                dolphin_path: PathBuf::new(),
            },
            dirty: false,
        };

        mock_state.data.presets.insert(
            "test_id".to_string(),
            Preset {
                id: "test_id".to_string(),
                created_at: UtcDateTime::now(),
                name: "Test Preset".to_string(),
                game_path: PathBuf::from("/path/to/game.iso"),
                xml_path: PathBuf::from("/path/to/riivolution/test.xml"),
                section_name: "Test Section".to_string(),
                options: vec![
                    PresetOpt {
                        name: "Option 1".to_string(),
                        selected: "Choice 1".to_string(),
                        choices: vec!["Choice 1".to_string(), "Choice 2".to_string()],
                    },
                ],
            },
        );

        app.manage(Mutex::new(mock_state));

        let state = app.state::<TauriState>();
        let state = state.lock().unwrap();
        let result = create_json(&state.data, "test_id");

        assert!(result.is_ok());
    }
}

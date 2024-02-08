use std::fs::File;
use std::io::{ Read, Write};
use std::rc::Rc;
use serde_json;
use crate::app_state::AppState;
use dirs::home_dir;
const CONFIG_FILE: &str = ".settings-cherrymx.json";


pub fn save_settings(app_state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    let serialized = serde_json::to_string(app_state)?;
    let path = config_file_path();
    println!("Saving settings to {}", path);
    let mut file = File::create(path)?;

    write!(file, "{}", serialized)?;
    Ok(())
}

pub fn read_settings(app_state: Rc<AppState>) -> Option<AppState> {
    let path = config_file_path();
    let f = File::open(path);

    if let Ok(mut fh) = f {
        let mut saved_settings = String::new();

        fh.read_to_string(&mut saved_settings)
            .expect("Unable to read saved command");

        let settings: AppState = serde_json::from_str(&saved_settings).expect("Unable to use saved command");

        app_state.update_mode(settings.mode_value.take());
        app_state.update_color(settings.color_value.take());
        app_state.update_speed(settings.scale_value.take());

        return Some(settings);
    }
    None
}

fn config_file_path() -> String {
    match home_dir() {
        Some(path) => format!("{}/{}", path.to_string_lossy(), CONFIG_FILE),
        None => String::new(),
    }
}
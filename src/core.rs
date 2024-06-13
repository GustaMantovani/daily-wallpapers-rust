// Copyright 2024 Gustavo Mantovani
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// src/core.rs

use crate::models::{DwConfig, DwWallpaperCandidate};
use chrono::Local;
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
    process::Command,
};

pub fn change_wallpaper(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        //gsettings not return error for inexistent paths, so we produce this
        return Err("The specified file path does not exist.".into());
    }

    let command = format!(
        "gsettings set org.gnome.desktop.background picture-uri-dark {:?}",
        path
    );

    let command_execution_output = Command::new("sh")
        .args(["-c", &command])
        .output()
        .map_err(|e| format!("Failed to execute process: {}", e))?;

    if !command_execution_output.status.success() {
        return Err("The command to change the wallpaper failed.".into());
    }

    return Ok(());
}

pub fn read_config_json(path: &String) -> Result<DwConfig, Box<dyn Error>> {
    let contents =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read file {}: {}", path, e))?;
    let config: DwConfig =
        serde_json::from_str(&contents).map_err(|e| format!("Failed to parse JSON: {}", e))?;
    Ok(config)
}

pub fn write_config_json(config: DwConfig, path: String) -> Result<(), Box<dyn Error>> {
    let json_data = serde_json::to_string_pretty(&config)?;
    let mut file = File::create(path)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    const PATH: &str = "./config/config.json";

    let mkdir_config_execution_output = Command::new("sh")
        .args(["-c", "mkdir -p config"])
        .output()
        .map_err(|e| format!("Failed to execute process: {}", e))?;

    if !mkdir_config_execution_output.status.success() {
        return Err("The command to crate config directory failed.".into());
    }

    let touch_config_json_execution_output = Command::new("sh")
        .args(["-c", "touch config/config.json"])
        .output()
        .map_err(|e| format!("Failed to execute process: {}", e))?;

    if !touch_config_json_execution_output.status.success() {
        return Err("The command to crate config.json file failed.".into());
    }

    let empty_config = DwConfig {
        actual_wallpaper: DwWallpaperCandidate {
            path: "".to_string(),
            date_set: Local::now(),
            child: false,
        },
        preset: crate::models::DwPreset::DAY,
        candidates: Vec::new(),
    };
    return write_config_json(empty_config, PATH.to_string());
}

pub fn change_config_file(new_config_path: &Path) -> Result<(), Box<dyn std::error::Error>> {

    match read_config_json(&format!("{new_config_path:?}")) {
        Ok(_) => {
            const PATH: &str = "./config/config.json";

            let res_output_mv_new_config_file = Command::new("sh")
                .args(["-c", &format!("mv {new_config_path:?} {PATH}")])
                .output()
                .map_err(|e| format!("Failed to execute process: {}", e))?;

            if !res_output_mv_new_config_file.status.success() {
                return Err("The command to set new config file failed".into());
            }

            return Ok(());
        }
        Err(e) => {
            return Err(e.into());
        }
    }
}

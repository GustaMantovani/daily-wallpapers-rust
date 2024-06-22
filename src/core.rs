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

use crate::models::{DwConfig, DwPreset, DwTimeConfig, DwWallpaperCandidate};
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
        // gsettings not return errors for inexistent paths, so we need produce it
        return Err("Error: The specified file path does not exist.".into());
    }

    if tree_magic::from_filepath(path).split("/").next() != Some("image"){
        return Err("Err: file is not an image".into());
    }

    let command = format!(
        "gsettings set org.gnome.desktop.background picture-uri-dark {:?}",
        path
    );

    let command_execution_output =
        Command::new("sh")
            .args(["-c", &command])
            .output()
            .map_err(|e| {
                format!(
                    "Error: Failed to execute process to change wallpaper: {}",
                    e
                )
            })?;

    if !command_execution_output.status.success() {
        return Err("Error: The command to change the wallpaper failed.".into());
    }

    Ok(())
}

pub fn read_config_json(path: &str) -> Result<DwConfig, Box<dyn Error>> {
    let contents = fs::read_to_string(path)
        .map_err(|e| format!("Error: Failed to read file {}: {}", path, e))?;
    let config: DwConfig = serde_json::from_str(&contents)
        .map_err(|e| format!("Error: Failed to parse JSON in file {}: {}", path, e))?;
    Ok(config)
}

pub fn write_config_json(config: DwConfig, path: String) -> Result<(), Box<dyn Error>> {
    let json_data = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Error: Failed to serialize config to JSON: {}", e))?;
    let mut file =
        File::create(&path).map_err(|e| format!("Error: Failed to create file {}: {}", path, e))?;
    file.write_all(json_data.as_bytes())
        .map_err(|e| format!("Error: Failed to write JSON data to file {}: {}", path, e))?;
    Ok(())
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    const PATH: &str = "./config/config.json";

    Command::new("sh")
        .args(["-c", "mkdir -p config"])
        .output()
        .map_err(|e| format!("Error: Failed to create config directory: {}", e))?;
    Command::new("sh")
        .args(["-c", "touch config/config.json"])
        .output()
        .map_err(|e| format!("Error: Failed to create config.json file: {}", e))?;
    let empty_config = DwConfig {
        actual_wallpaper: DwWallpaperCandidate {
            index: 0,
            path: "".to_string(),
            date_set: Local::now(),
            child: false,
            sub_index: 0,
        },
        time_config: DwTimeConfig {
            preset: DwPreset::DAY,
            interval: 1,
        },
        candidates: Vec::new(),
    };
    write_config_json(empty_config, PATH.to_string())?;

    Ok(())
}

pub fn change_config_file(new_config_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    read_config_json(new_config_path.to_str().unwrap()).map_err(|e| {
        format!(
            "Error: Failed to read new config file at {}: {}",
            new_config_path.display(),
            e
        )
    })?;

    const CONFIG_DIR_PATH: &str = "./config";

    Command::new("sh")
        .args(["-c", &format!("mkdir -p {}", CONFIG_DIR_PATH)])
        .output()
        .map_err(|e| {
            format!(
                "Error: Failed to create config directory {}: {}",
                CONFIG_DIR_PATH, e
            )
        })?;

    Command::new("sh")
        .args([
            "-c",
            &format!("cp {:?} {}", new_config_path, CONFIG_DIR_PATH),
        ])
        .output()
        .map_err(|e| {
            format!(
                "Error: Failed to copy new config file to {}: {}",
                CONFIG_DIR_PATH, e
            )
        })?;

    Ok(())
}

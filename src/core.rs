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

use std::{error::Error, fs, path::Path, process::Command};

use crate::models::DwConfig;
use std::fs::File;
use std::io::Write;

pub fn change_wallpaper(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
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

    if command_execution_output.status.success() {
        Ok(())
    } else {
        Err("The command to change the wallpaper failed.".into())
    }
}

pub fn read_config_json() -> Result<DwConfig, Box<dyn Error>> {
    let path = "./config/config.json";

    let contents =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read file {}: {}", path, e))?;
    let config: DwConfig =
        serde_json::from_str(&contents).map_err(|e| format!("Failed to parse JSON: {}", e))?;
    Ok(config)
}

pub fn write_config_json(config: DwConfig) -> Result<(), Box<dyn Error>> {
    let path = "./config/config.json";

    let json_data = serde_json::to_string_pretty(&config)?;
    let mut file = File::create(path)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

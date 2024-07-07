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

use crate::core_models::{DwConfig, DwPreset, DwTimeConfig, DwWallpaperCandidate};
use chrono::Local;
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    env::consts::OS,
};

pub fn change_wallpaper(path: &Path) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        return Err("Error: The specified file path does not exist.".into());
    }

    if tree_magic::from_filepath(path).split('/').next() != Some("image") {
        return Err("Error: The file is not an image.".into());
    }

    let os_type = OS;
    let result = match os_type {
        "linux" => set_linux_wallpaper(path),
        "windows" => set_windows_wallpaper(path),
        "macos" => set_macos_wallpaper(path),
        _ => Err("Error: Unsupported operating system.".into()),
    };

    result
}

fn set_linux_wallpaper(path: &Path) -> Result<(), Box<dyn Error>> {
    let desktop_env = get_desktop_environment();

    let command = match desktop_env.as_str() {
        "gnome" => format!("gsettings set org.gnome.desktop.background picture-uri-dark {:?}", path),
        "kde" => format!("qdbus org.kde.plasmashell /PlasmaShell org.kde.PlasmaShell.evaluateScript 'var Desktops = desktops(); for (i=0; i<Desktops.length; i++) {{d = Desktops[i];d.wallpaperPlugin = \"org.kde.image\";d.currentConfigGroup = Array(\"Wallpaper\", \"org.kde.image\", \"General\");d.writeConfig(\"Image\", \"file://{}\")}}'", path.to_string_lossy()),
        "xfce" => format!("xfconf-query --channel xfce4-desktop --property /backdrop/screen0/monitor0/image-path --set {:?}", path),
        _ => return Err("Error: Unsupported Linux desktop environment.".into()),
    };

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

fn set_windows_wallpaper(path: &Path) -> Result<(), Box<dyn Error>> {
    // Converte o caminho para uma string
    let wallpaper_path = path.to_str().ok_or("Invalid path")?;
    // Executa o comando
    let command_execution_output = Command::new("external_builds\\windows\\WallpaperChanger.exe")
        // .args(["/C", &format!(
        //     "external_builds\\windows\\WallpaperChanger.exe {}",
        //     wallpaper_path
        // )])
        .args(Some(wallpaper_path))
        .output()
        .map_err(|e| {
            format!(
                "Error: Failed to execute process to change wallpaper: {}",
                e
            )
        })?;

    // Verifica se o comando foi bem-sucedido
    if !command_execution_output.status.success() {
        return Err("Error: The command to change the wallpaper failed.".into());
    }

    Ok(())
}

fn set_macos_wallpaper(path: &Path) -> Result<(), Box<dyn Error>> {
    let command = format!(
        "osascript -e 'tell application \"System Events\" to set picture of every desktop to {:?}'",
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

fn get_desktop_environment() -> String {
    if let Ok(desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
        desktop.to_lowercase()
    } else if let Ok(desktop) = std::env::var("DESKTOP_SESSION") {
        desktop.to_lowercase()
    } else {
        String::new()
    }
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

pub fn found_wpp_path_by_index_in_directory(dir_path: &Path, index: usize) -> Result<String, Box<dyn Error>> {
    // Check if the directory exists
    if !dir_path.is_dir() {
        return Err(format!("The specified path is not a directory: {}", dir_path.display()).into());
    }

    // Read the directory contents and filter only image files
    let mut paths: Vec<_> = fs::read_dir(dir_path)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| tree_magic::from_filepath(path).starts_with("image/"))
        .collect();

    // Sort the paths
    paths.sort();

    // Check if the index is within the range of paths
    if index >= paths.len() {
        return Err(format!("Index {} is out of bounds for directory {}", index, dir_path.display()).into());
    }

    // Return the path corresponding to the index
    Ok(paths[index].to_string_lossy().to_string())
}

pub fn found_wpp_index_by_path_in_directory(dir_path: &Path, target_file_name: &str) -> Result<usize, Box<dyn Error>> {
    // Check if the directory exists
    if !dir_path.is_dir() {
        return Err(format!("The specified path is not a directory: {}", dir_path.display()).into());
    }

    // Read the directory contents and filter only image files
    let mut paths: Vec<PathBuf> = fs::read_dir(dir_path)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| tree_magic::from_filepath(path).starts_with("image/"))
        .collect();

    // Sort the paths
    paths.sort();

    // Find the index of the target file name
    match paths.iter().position(|path| {
        if let Some(file_name) = path.file_name() {
            file_name == target_file_name
        } else {
            false
        }
    }) {
        Some(index) => Ok(index),
        None => Err(format!("File {} not found in directory {}", target_file_name, dir_path.display()).into())
    }
}

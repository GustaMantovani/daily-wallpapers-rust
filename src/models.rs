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

// src/models.rs

use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DwOperationExecutionResult {
    pub success: bool,
    pub exit_code: i32,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct DwWallpaperCandidate {
    pub path: String,
    pub date_set: DateTime<Local>,
    pub child: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DwPreset {
    HOUR,
    MINUTE,
    DAY,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DwConfig {
    pub actual_wallpaper: DwWallpaperCandidate,
    pub preset: DwPreset,
    pub candidates: Vec<String>,
}

// Clap models
#[derive(Parser, Debug)]
#[command(name = "dw", about = "Daily Wallpaper Manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Add a wallpaper or directory to wallpapers cycle")]
    AddWallpaper {
        #[arg()]
        path: String,
    },
    #[command(about = "Remove a wallpaper or directory from wallpapers cycle")]
    RemoveWallpaper {
        #[arg()]
        path: String,
    },
    #[command(about = "Set time preset to wallpaper change")]
    Preset {
        #[arg()]
        preset: String,
        #[arg(
            required_if_eq("preset", "by minutes"),
            required_if_eq("preset", "by hours")
        )]
        interval: Option<u64>,
    },
    #[command(about = "Set the first wallpaper in the cycle and reset")]
    Reset,
    #[command(about = "Sets a specific wallpaper, but does not change the cycle")]
    SetWallpaper { path: String },
    #[command(about = "Set the next wallpaper in the cycle")]
    Next,
    #[command(about = "Disable daily wallpapers")]
    Off,
    #[command(about = "Enable daily wallpapers")]
    On,
    #[command(about = "Shows daily wallpapers config.json")]
    ShowConfig,
    #[command(
        about = "Sets a json file as the config file of the program. If json file is not present in argument, this commands will create an empty config.json"
    )]
    SetConfig { path: String },
    #[command(about = "Performs first time setup")]
    Init,
}

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
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DwOperationExecutionResult {
    pub success: bool,
    pub exit_code: i32,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct DwWallpaperCandidate {
    pub index: usize,
    pub path: String,
    pub date_set: DateTime<Local>,
    pub child: bool,
    pub sub_index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DwPreset {
    HOUR,
    MINUTE,
    DAY,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DwTimeConfig {
    pub preset: DwPreset,
    pub interval: u8,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DwConfig {
    pub actual_wallpaper: DwWallpaperCandidate,
    pub time_config: DwTimeConfig,
    pub candidates: Vec<String>,
}

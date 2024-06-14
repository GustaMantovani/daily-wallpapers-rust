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

// src/operations.rs

use crate::core::{
    change_config_file, change_wallpaper, init, read_config_json, write_config_json,
};
use crate::models::{DwOperationExecutionResult, DwPreset};
use std::path::Path;

pub fn set_wallpaper(path: &String) -> DwOperationExecutionResult {
    match change_wallpaper(Path::new(path)) {
        Ok(_) => DwOperationExecutionResult {
            success: true,
            exit_code: 0,
            message: None,
        },
        Err(err) => DwOperationExecutionResult {
            success: false,
            exit_code: 1,
            message: Some(err.to_string()),
        },
    }
}

pub fn show_config() -> DwOperationExecutionResult {
    match read_config_json(&"./config/config.json".to_string()) {
        Ok(config) => {
            return DwOperationExecutionResult {
                success: true,
                exit_code: 0,
                message: Some(serde_json::to_string_pretty(&config).unwrap()),
            };
        }
        Err(e) => {
            return DwOperationExecutionResult {
                success: false,
                exit_code: 2,
                message: Some(e.to_string()),
            };
        }
    }
}

pub fn perform_init() -> DwOperationExecutionResult {
    match init() {
        Ok(_) => {
            return DwOperationExecutionResult {
                success: true,
                exit_code: 0,
                message: None,
            };
        }
        Err(e) => {
            return DwOperationExecutionResult {
                success: false,
                exit_code: 3,
                message: Some(e.to_string()),
            };
        }
    }
}

pub fn set_config(path: &String) -> DwOperationExecutionResult {
    match change_config_file(Path::new(&path)) {
        Ok(_) => {
            return DwOperationExecutionResult {
                success: true,
                exit_code: 0,
                message: None,
            };
        }

        Err(e) => {
            return DwOperationExecutionResult {
                success: false,
                exit_code: 4,
                message: Some(e.to_string()),
            };
        }
    }
}

pub fn add_wallpaper(path: &String) -> DwOperationExecutionResult {
    if !Path::new(path).exists() {
        return DwOperationExecutionResult {
            success: false,
            exit_code: 2,
            message: Some("The specified file or directory does not exist".to_string()),
        };
    }

    match read_config_json(&"config/config.json".to_string()) {
        Ok(mut config) => {
            config.candidates.push(path.clone());

            match write_config_json(config, "config/config.json".to_string()) {
                Ok(_) => {
                    return DwOperationExecutionResult {
                        success: true,
                        exit_code: 0,
                        message: None,
                    };
                }
                Err(e) => {
                    return DwOperationExecutionResult {
                        success: false,
                        exit_code: 5,
                        message: Some(e.to_string()),
                    };
                }
            }
        }
        Err(e) => {
            return DwOperationExecutionResult {
                success: false,
                exit_code: 6,
                message: Some(e.to_string()),
            };
        }
    }
}

pub fn rm_wallpaper(path: &String) -> DwOperationExecutionResult {
    match read_config_json(&"config/config.json".to_string()) {
        Ok(mut config) => {
            if let Some(index) = config.candidates.iter().position(|x| x == path) {
                config.candidates.remove(index);

                match write_config_json(config, "config/config.json".to_string()) {
                    Ok(_) => {
                        return DwOperationExecutionResult {
                            success: true,
                            exit_code: 0,
                            message: None,
                        };
                    }
                    Err(e) => {
                        return DwOperationExecutionResult {
                            success: false,
                            exit_code: 5,
                            message: Some(e.to_string()),
                        };
                    }
                }
            } else {
                return DwOperationExecutionResult {
                    success: false,
                    exit_code: 7,
                    message: Some("Wallpaper not found in config".to_string()),
                };
            }
        }
        Err(e) => {
            return DwOperationExecutionResult {
                success: false,
                exit_code: 6,
                message: Some(e.to_string()),
            };
        }
    }
}

pub fn set_preset(preset: &str, interval: Option<u8>) -> DwOperationExecutionResult {
    match read_config_json(&"config/config.json".to_string()) {
        Ok(mut config) => {
            let enum_preset;

            match preset {
                "minute" => {
                    enum_preset = DwPreset::MINUTE;
                }
                "hour" => {
                    enum_preset = DwPreset::HOUR;
                }
                "day" => {
                    enum_preset = DwPreset::DAY;
                }
                _ => {
                    return DwOperationExecutionResult {
                        success: false,
                        exit_code: 9,
                        message: Some("Invalid preset".to_string()),
                    };
                }
            };

            config.time_config.preset = enum_preset;

            if interval != None {
                config.time_config.interval = interval.unwrap();
            }

            match write_config_json(config, "./config/config.json".to_string()) {
                Ok(_) => {
                    return DwOperationExecutionResult {
                        success: true,
                        exit_code: 0,
                        message: None,
                    };
                }
                Err(e) => {
                    return DwOperationExecutionResult {
                        success: false,
                        exit_code: 11,
                        message: Some(e.to_string()),
                    };
                }
            }
        }
        Err(e) => {
            return DwOperationExecutionResult {
                success: false,
                exit_code: 8,
                message: Some(e.to_string()),
            };
        }
    }
}

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

use std::fs;
use tree_magic;

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
            }
            return DwOperationExecutionResult {
                success: false,
                exit_code: 7,
                message: Some("Wallpaper not found in config".to_string()),
            };
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

pub fn previous() -> DwOperationExecutionResult {
    match read_config_json("config/config.json") {
        Ok(config) => {
            if config.actual_wallpaper.child {
                if let Some((dir, _file)) = config.actual_wallpaper.path.rsplit_once("/") {
                    println!("{dir:?} | {}", config.candidates[config.actual_wallpaper.index]);
                    if Path::new(dir) == Path::new(&config.candidates[config.actual_wallpaper.index]) {
                        match fs::read_dir(dir) {
                            Ok(dir) => {
                                for entry in dir {
                                    if let Ok(entry) = entry {
                                        let path = entry.path();
                                        if path.is_file() {
                                            let mime_type = tree_magic::from_filepath(path.as_path());
                                            let mime_type_only = mime_type.split("/").next();
                                            if let Some(mime_type_only) = mime_type_only {
                                                if mime_type_only == "image" {
                                                    println!("{:?} is an image", entry.file_name());
                                                }
                                            }
                                        }
                                    }
                                }
                                return DwOperationExecutionResult {
                                    success: false,
                                    exit_code: 19,
                                    message: Some("Erro ao procurar o caminho para o candidato pai da entrada atual".into()),
                                };
                            }
                            Err(e) => {
                                return DwOperationExecutionResult {
                                    success: false,
                                    exit_code: 16,
                                    message: Some(format!("{e}")),
                                };
                            }
                        }
                    }
                } else {
                    return DwOperationExecutionResult {
                        success: false,
                        exit_code: 17,
                        message: Some("Erro ao procurar o caminho para o candidato pai da entrada atual".into()),
                    };
                }
            }
            return DwOperationExecutionResult {
                success: false,
                exit_code: 15,
                message: Some("Erro ao procurar o caminho para o candidato pai da entrada atual".into()),
            };
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
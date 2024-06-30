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
use crate::core_models::{DwOperationExecutionResult, DwPreset};
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
        Err(err) => {
            return DwOperationExecutionResult {
                success: false,
                exit_code: 2,
                message: Some(err.to_string()),
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
            exit_code: 5,
            message: Some("The specified file or directory does not exist".to_string()),
        };
    }

    if tree_magic::from_filepath(Path::new(path)).split("/").next() != Some("image") {
        return DwOperationExecutionResult {
            success: false,
            exit_code: 17,
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
                        exit_code: 6,
                        message: Some(e.to_string()),
                    };
                }
            }
        }
        Err(e) => {
            return DwOperationExecutionResult {
                success: false,
                exit_code: 7,
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
                            exit_code: 8,
                            message: Some(e.to_string()),
                        };
                    }
                }
            }
            return DwOperationExecutionResult {
                success: false,
                exit_code: 9,
                message: Some("Wallpaper not found in config".to_string()),
            };
        }
        Err(e) => {
            return DwOperationExecutionResult {
                success: false,
                exit_code: 10,
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
                        exit_code: 11,
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
                        exit_code: 12,
                        message: Some(e.to_string()),
                    };
                }
            }
        }
        Err(e) => {
            return DwOperationExecutionResult {
                success: false,
                exit_code: 13,
                message: Some(e.to_string()),
            };
        }
    }
}

pub fn previous() -> DwOperationExecutionResult {
    match read_config_json("config/config.json") {
        Ok(config) => {
            let previous_wallpaper_path: String;

            if config.actual_wallpaper.child {
                if let Some((wallpaper_dir_path, wallpaper_file_name)) =
                    config.actual_wallpaper.path.rsplit_once("/")
                {
                    if config.actual_wallpaper.sub_index == 0 {
                        if config.actual_wallpaper.path
                            == config.candidates[config.actual_wallpaper.index as usize]
                        {
                            match config
                                .candidates
                                .get((config.actual_wallpaper.index - 1) as usize)
                            {
                                Some(_) => {
                                    previous_wallpaper_path = format!(
                                        "{}",
                                        config.candidates
                                            [(config.actual_wallpaper.index - 1) as usize]
                                    );
                                }
                                None => {
                                    previous_wallpaper_path = format!(
                                        "{}",
                                        config.candidates
                                            [(config.actual_wallpaper.index - 1) as usize]
                                    );
                                }
                            }
                        } else {
                            previous_wallpaper_path = format!(
                                "{}",
                                config.candidates[config.actual_wallpaper.index as usize]
                            );
                        }

                        return set_wallpaper(&previous_wallpaper_path);
                    }

                    println!("{wallpaper_dir_path}");
                    if Path::new(wallpaper_dir_path)
                        == Path::new(&config.candidates[config.actual_wallpaper.index as usize])
                    {
                        match fs::read_dir(wallpaper_dir_path) {
                            Ok(dirs) => {
                                let mut paths: Vec<_> = dirs
                                    .filter_map(|entry| entry.ok())
                                    .map(|entry| entry.path())
                                    .collect();

                                paths.sort();

                                if wallpaper_file_name
                                    == paths[config.actual_wallpaper.sub_index as usize]
                                        .file_name()
                                        .unwrap()
                                {
                                    previous_wallpaper_path = format!(
                                        "{}{}",
                                        wallpaper_dir_path,
                                        paths[(config.actual_wallpaper.sub_index - 1) as usize]
                                            .file_name()
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                            .to_string(),
                                    );
                                } else {
                                    previous_wallpaper_path = format!(
                                        "{}{}",
                                        wallpaper_dir_path,
                                        paths[config.actual_wallpaper.sub_index as usize]
                                            .file_name()
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                            .to_string(),
                                    );
                                }
                                return set_wallpaper(&previous_wallpaper_path);
                            }
                            Err(e) => {
                                return DwOperationExecutionResult {
                                    success: false,
                                    exit_code: 14,
                                    message: Some(e.to_string()),
                                };
                            }
                        }
                    }
                }
                return DwOperationExecutionResult {
                    success: false,
                    exit_code: 15,
                    message: Some("Error getting parent candidate path from entry".into()),
                };
            }

            return DwOperationExecutionResult {
                success: true,
                exit_code: 0,
                message: None,
            };
        }
        Err(e) => {
            return DwOperationExecutionResult {
                success: false,
                exit_code: 16,
                message: Some(e.to_string()),
            };
        }
    }
}

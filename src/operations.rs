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

use chrono::Local;

use crate::core_functions::{
    change_config_file, change_wallpaper, found_wpp_index_by_path_in_directory,
    found_wpp_path_by_index_in_directory, init, list_images_in_directory, read_config_json,
    write_config_json,
};
use crate::core_models::{DwOperationExecutionResult, DwPreset};

use std::{
    fs,
    path::{Path, MAIN_SEPARATOR},
    usize,
};

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

    if tree_magic::from_filepath(Path::new(path)).split("/").next() != Some("image")
        && !Path::new(path).is_dir()
    {
        return DwOperationExecutionResult {
            success: false,
            exit_code: 17,
            message: Some(
                "The specified file or directory does isn't an image or directory".to_string(),
            ),
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
        Ok(mut config) => {
            let previous_wallpaper_path: String;
            let mut previous_wallpaper_index: u8 = config.actual_wallpaper.index;
            let mut previous_wallpaper_child: bool = config.actual_wallpaper.child;
            let mut previous_wallpaper_sub_index: u8;

            if config.actual_wallpaper.child {
                let wallpaper_path = Path::new(&config.actual_wallpaper.path);
                if let Some(wallpaper_dir_path) = wallpaper_path.parent() {
                    if let Some(wallpaper_file_name) = wallpaper_path.file_name() {
                        println!(
                            "\n{} \t {}\n",
                            wallpaper_dir_path.display(),
                            wallpaper_file_name.to_string_lossy()
                        );

                        match found_wpp_index_by_path_in_directory(
                            wallpaper_dir_path,
                            &wallpaper_file_name.to_string_lossy(),
                        ) {
                            Ok(i) => {
                                if i == 0 {
                                    previous_wallpaper_index -= 1;

                                    if Path::new(
                                        &config.candidates
                                            [(config.actual_wallpaper.index - 1) as usize],
                                    )
                                    .is_dir()
                                    {
                                        match list_images_in_directory( Path::new(&config.candidates[(config.actual_wallpaper.index - 1) as usize])) {
                                            Ok(image_paths) => {
                                                previous_wallpaper_path =
                                                    image_paths.last().unwrap().clone();
                                                previous_wallpaper_child = true;
                                                previous_wallpaper_sub_index = (image_paths.len() - 1) as u8;
                                            }
                                            Err(e) => {
                                                return DwOperationExecutionResult {
                                                    success: false,
                                                    exit_code: 16,
                                                    message: Some(e.to_string()),
                                                };
                                            }
                                        }
                                        println!("{}", previous_wallpaper_path);
                                    } else {
                                        previous_wallpaper_path = config.candidates
                                            [(config.actual_wallpaper.index - 1) as usize]
                                            .clone();
                                        previous_wallpaper_sub_index = found_wpp_index_by_path_in_directory(Path::new("config/config.json"), &config.candidates[(config.actual_wallpaper.index - 1) as usize]).unwrap() as u8;
                                    }
                                } else {
                                    previous_wallpaper_sub_index = (i - 1) as u8;
                                    previous_wallpaper_path = found_wpp_path_by_index_in_directory(
                                        wallpaper_dir_path,
                                        i - 1,
                                    )
                                    .unwrap()
                                    .to_string();
                                }
                            }
                            Err(e) => {
                                return DwOperationExecutionResult {
                                    success: false,
                                    exit_code: 16,
                                    message: Some(e.to_string()),
                                };
                            }
                        }
                    } else {
                        return DwOperationExecutionResult {
                            success: false,
                            exit_code: 15,
                            message: Some("Error getting file name from path".into()),
                        };
                    }
                } else {
                    return DwOperationExecutionResult {
                        success: false,
                        exit_code: 15,
                        message: Some("Error getting parent directory from path".into()),
                    };
                }

                config.actual_wallpaper.path = previous_wallpaper_path.clone();
                config.actual_wallpaper.date_set = Local::now();
                config.actual_wallpaper.index = previous_wallpaper_index;
                config.actual_wallpaper.child = previous_wallpaper_child;
                config.actual_wallpaper.sub_index = previous_wallpaper_sub_index;

                match write_config_json(config, "./config/config.json".into()) {
                    Ok(_) => {
                        return set_wallpaper(&previous_wallpaper_path);
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

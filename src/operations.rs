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
    write_config_json, generate_schedule
};
use crate::core_models::{DwOperationExecutionResult, DwPreset};
use std::{
    path::Path,
    usize,
    env,
    process::Command
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
            exit_code: 6,
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
                            exit_code: 7,
                            message: Some(e.to_string()),
                        };
                    }
                }
            }
            return DwOperationExecutionResult {
                success: false,
                exit_code: 8,
                message: Some("Wallpaper not found in config".to_string()),
            };
        }
        Err(e) => {
            return DwOperationExecutionResult {
                success: false,
                exit_code: 9,
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
                        exit_code: 10,
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
                exit_code: 12,
                message: Some(e.to_string()),
            };
        }
    }
}

pub fn previous() -> DwOperationExecutionResult {
    let mut config = match read_config_json("config/config.json") {
        Ok(config) => config,
        Err(e) => {
            return DwOperationExecutionResult {
                success: false,
                exit_code: 13,
                message: Some(e.to_string()),
            };
        }
    };

    //Config atual
    let actual_wallpaper_path: &Path = Path::new(&config.actual_wallpaper.path);
    let actual_wallpaper_index: usize = config.actual_wallpaper.index;
    let actual_wallpaper_child: bool = config.actual_wallpaper.child;

    //Tentando obter informações
    let (actual_wallpaper_dir_path, actual_wallpaper_file_name) =
        match actual_wallpaper_path.parent() {
            Some(dir_path) => match actual_wallpaper_path.file_name() {
                Some(file_name) => (dir_path, file_name),
                None => {
                    return DwOperationExecutionResult {
                        success: false,
                        exit_code: 14,
                        message: Some("Error getting file name from path".into()),
                    };
                }
            },
            None => {
                return DwOperationExecutionResult {
                    success: false,
                    exit_code: 15,
                    message: Some("Error getting parent directory from path".into()),
                };
            }
        };



    //Config pós operação
    let previous_wallpaper_path: String;
    let previous_wallpaper_index: usize;
    let previous_wallpaper_child: bool;
    let mut previous_wallpaper_sub_index: usize;

    if actual_wallpaper_child {

        //Tentando obter o subindex
        let actual_wallpaper_sub_index = match found_wpp_index_by_path_in_directory(
            actual_wallpaper_dir_path,
            actual_wallpaper_file_name.to_str().unwrap(),
        ) {
            Ok(index) => index,
            Err(_) => config.actual_wallpaper.sub_index
        };

        //Conseguindo o subindex...
        if actual_wallpaper_sub_index == 0 {

            if actual_wallpaper_index == 0{
                previous_wallpaper_index = config.candidates.len() - 1;
            }else {
                previous_wallpaper_index = actual_wallpaper_index - 1; //Sabendo que vamos sair do conjunto atual de candidatos, temos que subtrair o index primário
            }

            //Se o subindex for zero, o conjunto de imagens atual muda, sendo necessário empregar lógica para defini-lo

            if Path::new(&config.candidates[previous_wallpaper_index]).is_dir() {
                previous_wallpaper_child = true; //Já temos certeza que o wallpaper anterior será um child

                //Tenta listar os candidatos na pasta anterior para definir o Wallpaper que será setado como a última imagem do diretório anterior (assim como seu subindex)
                match list_images_in_directory(Path::new(
                    &config.candidates[previous_wallpaper_index],
                )) {
                    Ok(image_paths) => {
                        previous_wallpaper_path = image_paths.last().unwrap().clone(); //O candidato anterior equivale à última imagem da pasta anterior
                        previous_wallpaper_sub_index = image_paths.len() - 1; //O subindex da última imagem do diretório anterior
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
                previous_wallpaper_child = false; //Aqui já sabemos que o anterior não é um diretório, pois esse else é um desvio condicional justamente disso
                previous_wallpaper_path = config.candidates[previous_wallpaper_index].clone();
                previous_wallpaper_sub_index = 0;
            }
        } else {
            previous_wallpaper_child = true; //Se não estamos saindo de um conjunto de imagens, o Wallpaper a ser setado ainda é um child
            previous_wallpaper_index = actual_wallpaper_index; //Se não estamos saindo de um conjunto, o index não muda
            previous_wallpaper_sub_index = actual_wallpaper_sub_index - 1; //Se o subindex do atual não for zero, significa que o subindex de quem será setado é o atual menos 1 (porque queremos o anterior)
            previous_wallpaper_path = match found_wpp_path_by_index_in_directory(actual_wallpaper_dir_path, previous_wallpaper_sub_index){ //Obtendo o path da imagem anterior
                Err(_) => {
                    previous_wallpaper_sub_index = 0;
                    found_wpp_path_by_index_in_directory(actual_wallpaper_dir_path, previous_wallpaper_sub_index).unwrap().to_string()
                }
                Ok(path) => path
            }
        }   
    }else{
        if actual_wallpaper_index == 0{
            previous_wallpaper_index = config.candidates.len() - 1;
        }else{
            previous_wallpaper_index = actual_wallpaper_index - 1;
        }

        if Path::new(&config.candidates[previous_wallpaper_index]).is_dir(){
            previous_wallpaper_child = true;

            match list_images_in_directory(Path::new(
                &config.candidates[previous_wallpaper_index],
            )) {
                Ok(image_paths) => {
                    previous_wallpaper_path = image_paths.last().unwrap().clone();
                    previous_wallpaper_sub_index = image_paths.len() - 1;
                }
                Err(e) => {
                    return DwOperationExecutionResult {
                        success: false,
                        exit_code: 17,
                        message: Some(e.to_string()),
                    };
                }
            }

        }else{
            previous_wallpaper_child = false;
            previous_wallpaper_sub_index = 0;
            previous_wallpaper_path = config.candidates[previous_wallpaper_index].clone();
        }
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
                exit_code: 18,
                message: Some(e.to_string()),
            };
        }
    }
}

pub fn next()-> DwOperationExecutionResult{
    let mut config = match read_config_json("config/config.json") {
        Ok(config) => config,
        Err(e) => {
            return DwOperationExecutionResult {
                success: false,
                exit_code: 19,
                message: Some(e.to_string()),
            };
        }
    };

    //Config atual
    let actual_wallpaper_path: &Path = Path::new(&config.actual_wallpaper.path);
    let actual_wallpaper_index: usize = config.actual_wallpaper.index;
    let actual_wallpaper_child: bool = config.actual_wallpaper.child;

    //Tentando obter informações
    let (actual_wallpaper_dir_path, actual_wallpaper_file_name) =
        match actual_wallpaper_path.parent() {
            Some(dir_path) => match actual_wallpaper_path.file_name() {
                Some(file_name) => (dir_path, file_name),
                None => {
                    return DwOperationExecutionResult {
                        success: false,
                        exit_code: 20,
                        message: Some("Error getting file name from path".into()),
                    };
                }
            },
            None => {
                return DwOperationExecutionResult {
                    success: false,
                    exit_code: 21,
                    message: Some("Error getting parent directory from path".into()),
                };
            }
        };

    //Config pós operação
    let next_wallpaper_path: String;
    let next_wallpaper_index: usize;
    let next_wallpaper_child: bool;
    let next_wallpaper_sub_index: usize;

    if actual_wallpaper_child {

        //Tentando obter o subindex
        let mut actual_wallpaper_sub_index = match found_wpp_index_by_path_in_directory(
            actual_wallpaper_dir_path,
            actual_wallpaper_file_name.to_str().unwrap(),
        ) {
            Ok(index) => index,
            Err(_) => config.actual_wallpaper.sub_index
        };

        let num_of_candidates_in_dir = match list_images_in_directory(&Path::new(&config.candidates[actual_wallpaper_index])) {
            Ok(image_paths) => {
                image_paths.len()
            },
            Err(e) => {
                return DwOperationExecutionResult {
                    success: false,
                    exit_code: 22,
                    message: Some(e.to_string()),
                };
            }
        };

        //Considero essa condição como referência perdida, então acho melhor resetar o ciclo no diretório atual
        if actual_wallpaper_sub_index > num_of_candidates_in_dir{
            actual_wallpaper_sub_index = 0;
        }

        //Conseguindo o subindex...
        if actual_wallpaper_sub_index == num_of_candidates_in_dir - 1 {

            if actual_wallpaper_index == config.candidates.len() - 1{
                next_wallpaper_index = 0;
            }else {
                next_wallpaper_index = actual_wallpaper_index + 1; //Sabendo que vamos sair do conjunto atual de candidatos, temos que subtrair o index primário
            }
            
            next_wallpaper_sub_index = 0;

            if Path::new(&config.candidates[next_wallpaper_index]).is_dir() {
                next_wallpaper_path = match found_wpp_path_by_index_in_directory(Path::new(&config.candidates[next_wallpaper_index]), 0){
                    Ok(path) => path,
                    Err(e) => {
                        return DwOperationExecutionResult {
                            success: false,
                            exit_code: 23,
                            message: Some(e.to_string()),
                        }
                    }
                };
                next_wallpaper_child = true; //Já temos certeza que o wallpaper anterior será um child

            } else {
                next_wallpaper_path = config.candidates[next_wallpaper_index].clone();
                next_wallpaper_child = false; //Aqui já sabemos que o posterior não é um diretório, pois esse else é um desvio condicional justamente disso
            }

        } else {
            next_wallpaper_child = true; //Se não estamos saindo de um conjunto de imagens, o Wallpaper a ser setado ainda é um child
            next_wallpaper_index = actual_wallpaper_index; //Se não estamos saindo de um conjunto, o index não muda
            next_wallpaper_sub_index = actual_wallpaper_sub_index + 1; //Se o subindex do atual não for zero, significa que o subindex de quem será setado é o atual mais 1 (porque queremos o posterior)
            next_wallpaper_path = match found_wpp_path_by_index_in_directory(Path::new(&config.candidates[next_wallpaper_index]), next_wallpaper_sub_index){
                Ok(path) => path,
                Err(e) => {
                    return DwOperationExecutionResult {
                        success: false,
                        exit_code: 24,
                        message: Some(e.to_string()),
                    }
                }
            };
        }   
    }else{
        if actual_wallpaper_index == config.candidates.len() - 1{
            next_wallpaper_index = 0;
        }else{
            next_wallpaper_index = actual_wallpaper_index + 1;
        }

        next_wallpaper_path = match found_wpp_path_by_index_in_directory(Path::new(&config.candidates[next_wallpaper_index]), 0){
            Ok(path) => path,
            Err(e) => {
                return DwOperationExecutionResult {
                    success: false,
                    exit_code: 25,
                    message: Some(e.to_string()),
                }
            }
        };

        next_wallpaper_sub_index = 0;

        if Path::new(&config.candidates[next_wallpaper_index]).is_dir(){
            next_wallpaper_child = true;
        }else{
            next_wallpaper_child = false;
        }
    }

    config.actual_wallpaper.path = next_wallpaper_path.clone();
    config.actual_wallpaper.date_set = Local::now();
    config.actual_wallpaper.index = next_wallpaper_index;
    config.actual_wallpaper.child = next_wallpaper_child;
    config.actual_wallpaper.sub_index = next_wallpaper_sub_index;

    match write_config_json(config, "./config/config.json".into()) {
        Ok(_) => {
            return set_wallpaper(&next_wallpaper_path);
        }

        Err(e) => {
            return DwOperationExecutionResult {
                success: false,
                exit_code: 26,
                message: Some(e.to_string()),
            };
        }
    }
}

pub fn reset() -> DwOperationExecutionResult {
    let mut config = match read_config_json("config/config.json") {
        Ok(config) => config,
        Err(e) => {
            return DwOperationExecutionResult {
                success: false,
                exit_code: 27,
                message: Some(e.to_string()),
            };
        }
    };

    let reset_wallpaper_path: String;
    let reset_wallpaper_child: bool;
    let reset_wallpaper_sub_index: usize;

    if Path::new(&config.candidates[0]).is_dir() {
        reset_wallpaper_child = true;
        reset_wallpaper_sub_index = 0;
        reset_wallpaper_path = match list_images_in_directory(Path::new(&config.candidates[0])) {
            Ok(image_paths) => {
                if !image_paths.is_empty() {
                    image_paths[0].clone()
                } else {
                    return DwOperationExecutionResult {
                        success: false,
                        exit_code: 28,
                        message: Some("The directory is empty".to_string()),
                    };
                }
            }
            Err(e) => {
                return DwOperationExecutionResult {
                    success: false,
                    exit_code: 29,
                    message: Some(e.to_string()),
                };
            }
        };
    } else {
        reset_wallpaper_child = false;
        reset_wallpaper_sub_index = 0;
        reset_wallpaper_path = config.candidates[0].clone();
    }

    config.actual_wallpaper.path = reset_wallpaper_path.clone();
    config.actual_wallpaper.date_set = Local::now();
    config.actual_wallpaper.index = 0;
    config.actual_wallpaper.child = reset_wallpaper_child;
    config.actual_wallpaper.sub_index = reset_wallpaper_sub_index;

    match write_config_json(config, "./config/config.json".into()) {
        Ok(_) => set_wallpaper(&reset_wallpaper_path),
        Err(e) => DwOperationExecutionResult {
            success: false,
            exit_code: 30,
            message: Some(e.to_string()),
        },
    }
}

pub fn on() -> DwOperationExecutionResult{  
    let config = match read_config_json("config/config.json") {
        Ok(config) => config,
        Err(e) => {
            return DwOperationExecutionResult {
                success: false,
                exit_code: 31,
                message: Some(e.to_string()),
            };
        }
    };

    let action: String;

    #[cfg(target_os = "linux")]{
        action = "~/.dw/bin/dw next".to_string();
    }

    #[cfg(target_os = "windows")]
    {
        action = match env::var("USERPROFILE") {
            Ok(user_profile) => format!(
                "powershell -Command 'cd {}\\.dwr ; .\\daily-wallpapers-rust.exe next'",
                user_profile
            ),
            Err(e) => {
                return DwOperationExecutionResult {
                    success: false,
                    exit_code: 32,
                    message: Some(e.to_string()),
                };
            },
        };
    }
    
    
    
    // Converting `action` to `&str`
    let action: &str = &action;

    //println!("{}", scheduler_string);
    #[cfg(target_os = "linux")]
    {
        // Chama a função `generate_schedule` e obtém a string do cron
        let cron_string = generate_schedule(config.time_config.preset, config.time_config.interval, "DWR", action);
    
        // Função para adicionar a entrada ao cron
        fn add_cron_entry(entry: &str) -> Result<(), Box<dyn std::error::Error>> {
            let status = Command::new("sh")
                .arg("scripts/linux/cron_adder.sh")
                .arg(entry)
                .status()?;
    
            if status.success() {
                Ok(())
            } else {
                Err("Failed to add cron entry".into())
            }
        }
    
        match config.time_config.preset {
            DwPreset::DAY => {
                return DwOperationExecutionResult {
                    success: false,
                    exit_code: 33,
                    message: Some("Unsupported preset".to_string()),
                };
            }
    
            DwPreset::HOUR | DwPreset::MINUTE => {
                match add_cron_entry(cron_string.as_str()) {
                    Ok(()) => {
                        return DwOperationExecutionResult {
                            success: true,
                            exit_code: 0,
                            message: Some("Cron entry added successfully".to_string()),
                        };
                    }
                    Err(e) => {
                        return DwOperationExecutionResult {
                            success: false,
                            exit_code: 34,
                            message: Some(e.to_string()),
                        };
                    }
                }
            }
    
            _ => {
                return DwOperationExecutionResult {
                    success: false,
                    exit_code: 35,
                    message: Some("Unsupported preset".to_string()),
                };
            }
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        let (command, args) = generate_schedule(config.time_config.preset, config.time_config.interval, "DWR", &action);
    
        let output = Command::new(&command)
            .args(&args)
            .output();
    
        match output {
            Ok(output) => {
                if output.status.success() {
                    return DwOperationExecutionResult {
                        success: true,
                        exit_code: 0,
                        message: Some("Command executed successfully".to_string()),
                    };
                } else {
                    return DwOperationExecutionResult {
                        success: false,
                        exit_code: output.status.code().unwrap_or(1),
                        message: Some(String::from_utf8_lossy(&output.stderr).to_string()),
                    };
                }
            }
            Err(e) => {
                return DwOperationExecutionResult {
                    success: false,
                    exit_code: 1,
                    message: Some(e.to_string()),
                };
            }
        }
    }
}

pub fn off() -> DwOperationExecutionResult {

    #[cfg(target_os = "linux")]{

        let config = match read_config_json("config/config.json") {
            Ok(config) => config,
            Err(e) => {
                return DwOperationExecutionResult {
                    success: false,
                    exit_code: 36,
                    message: Some(e.to_string()),
                };
            }
        };

        fn remove_cron_entry(entry: &str) -> Result<(), Box<dyn std::error::Error>> {
            let status = Command::new("sh")
                .arg("scripts/linux/cron_remover.sh")
                .arg(entry)
                .status()?;
            
            if status.success() {
                Ok(())
            } else {
                Err("Failed to add cron entry".into())
            }
        }
        
        let scheduler_string = generate_schedule(config.time_config.preset, config.time_config.interval, "DWR", "~/.dw/bin/dw next");

        match remove_cron_entry(scheduler_string.as_str()) {
            Ok(()) => {
                DwOperationExecutionResult {
                    success: true,
                    exit_code: 0,
                    message: Some("Entrada do cron removida com sucesso".to_string()),
                }
            }
            Err(e) => {
                DwOperationExecutionResult {
                    success: false,
                    exit_code: 37,
                    message: Some(format!("Erro ao remover a entrada do cron: {}", e)),
                }
            }
        }
    }

    #[cfg(target_os = "windows")]{
        let output = Command::new("cmd")
            .args(&["/C", "schtasks /delete /tn DWR /f"])
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    return DwOperationExecutionResult {
                        success: true,
                        exit_code: 38,
                        message: Some("Command executed successfully".to_string()),
                    };
                } else {
                    return DwOperationExecutionResult {
                        success: false,
                        exit_code: output.status.code().unwrap_or(1),
                        message: Some(String::from_utf8_lossy(&output.stderr).to_string()),
                    };
                }
            },
            Err(e) => {
                return DwOperationExecutionResult {
                    success: false,
                    exit_code: 39,
                    message: Some(e.to_string()),
                };
            }
        }   
    }
}

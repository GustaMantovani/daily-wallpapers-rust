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

// src/main.rs

mod clap_models;
mod core_functions;
mod core_models;
mod operations;
use crate::clap_models::{Cli, Commands};
use crate::operations::{
    add_wallpaper, next, off, on, perform_init, previous, reset, rm_wallpaper, set_config,
    set_preset, set_wallpaper, show_config,
};
use clap::Parser;
use std::{process::ExitCode, path::PathBuf, env};

fn main() -> ExitCode {

    //Define o contexto de diretório para a execução do programa
    if env!("CONTEXT_LOCAL") == "false" {
        let target_dir: PathBuf;

        #[cfg(target_os = "linux")]
        {
            if let Some(home_dir) = env::home_dir() {
                target_dir = home_dir.join(".dwr/");
            } else {
                eprintln!("Erro: Não foi possível determinar o diretório 'home' do usuário.");
                return ExitCode::FAILURE;
            }
        }
    
        #[cfg(target_os = "windows")]
        {
            match env::var("USERPROFILE") {
                Ok(user_profile) => {
                    target_dir = PathBuf::from(user_profile).join(".dwr\\");
                }
                Err(_) => {
                    eprintln!("Erro: Não foi possível determinar o diretório 'USERPROFILE'.");
                    return ExitCode::FAILURE;
                }
            };
        }
    
        // Tenta alterar o diretório de trabalho atual para `target_dir`
        if let Err(e) = env::set_current_dir(&target_dir) {
            println!("{:?}", target_dir);
            eprintln!("Erro ao definir o diretório: {}", e);
            return ExitCode::FAILURE;
        }
    }

    let cli: Cli = Cli::parse();
    let operation_res;

    match &cli.command {
        Commands::AddWallpaper { path } => {
            operation_res = add_wallpaper(path);
        }

        Commands::RmWallpaper { path } => {
            operation_res = rm_wallpaper(path);
        }

        Commands::Preset { preset, interval } => {
            operation_res = set_preset(&preset, *interval);
        }

        Commands::Next => {
            operation_res = next();
        }

        Commands::Previous => {
            operation_res = previous();
        }

        Commands::Reset => {
            operation_res = reset();
        }

        Commands::SetWallpaper { path } => {
            operation_res = set_wallpaper(path);
        }

        Commands::Off => {
            operation_res = off();
        }

        Commands::On => {
            operation_res = on();
        }

        Commands::ShowConfig => {
            operation_res = show_config();
        }

        Commands::SetConfig { path } => {
            operation_res = set_config(path);
        }

        Commands::Init => {
            operation_res = perform_init();
        }
    };

    if operation_res.success {
        println!("{:?}", operation_res);
        return ExitCode::SUCCESS;
    } else {
        eprintln!("{:?}", operation_res);
        return ExitCode::FAILURE;
    }
}

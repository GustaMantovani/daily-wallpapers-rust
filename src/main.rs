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

use std::process;

// External 👽️
use clap::{Parser, Subcommand};

// Our things 👥
mod dw_commands_operations;
mod dw_core_functions;
mod dw_models;

use dw_models::CoreFunctionExecutionResult;

#[derive(Parser, Debug)]
#[command(name = "dw", about = "Daily Wallpaper Manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
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
    Set { path: String },
    #[command(about = "Set the next wallpaper in the cycle")]
    Next,
    #[command(about = "Disable daily wallpapers")]
    Off,
    #[command(about = "Enable daily wallpapers")]
    On,
}

fn main() {
    let cli = Cli::parse();
    let res;

    match &cli.command {
        Commands::AddWallpaper { path } => {
            res = CoreFunctionExecutionResult {
                sucess: true,
                exit_code: 0,
                message: format!("Sucess"),
                command_execution_output: None,
            }
        }

        Commands::RemoveWallpaper { path } => {
            res = CoreFunctionExecutionResult {
                sucess: true,
                exit_code: 0,
                message: format!("Sucess"),
                command_execution_output: None,
            }
        }

        Commands::Preset { preset, interval } => {
            res = CoreFunctionExecutionResult {
                sucess: true,
                exit_code: 0,
                message: format!("Sucess"),
                command_execution_output: None,
            }
        }

        Commands::Next => {
            res = CoreFunctionExecutionResult {
                sucess: true,
                exit_code: 0,
                message: format!("Sucess"),
                command_execution_output: None,
            }
        }

        Commands::Reset => {
            res = CoreFunctionExecutionResult {
                sucess: true,
                exit_code: 0,
                message: format!("Sucess"),
                command_execution_output: None,
            }
        }

        Commands::Set { path } => res = dw_commands_operations::set_wallpaper(path),

        Commands::Off => {
            res = CoreFunctionExecutionResult {
                sucess: true,
                exit_code: 0,
                message: format!("Sucess"),
                command_execution_output: None,
            }
        }

        Commands::On => {
            res = CoreFunctionExecutionResult {
                sucess: true,
                exit_code: 0,
                message: format!("Sucess"),
                command_execution_output: None,
            }
        }
    };

    if !res.sucess {
        eprint!("{}", res.message);
        process::exit(res.exit_code as i32);
    }
}

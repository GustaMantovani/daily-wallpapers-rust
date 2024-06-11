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
use clap::Parser;

// Our things 👥
mod dw_commands_operations;
mod dw_core_functions;
mod dw_models;

use dw_models::DwExecuionResult;
use dw_models::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    let res;

    match &cli.command {
        Commands::AddWallpaper { path } => {
            res = DwExecuionResult {
                sucess: true,
                exit_code: 0,
                message: format!("Sucess"),
                sys_commando_execution_output: None,
            }
        }

        Commands::RemoveWallpaper { path } => {
            res = DwExecuionResult {
                sucess: true,
                exit_code: 0,
                message: format!("Sucess"),
                sys_commando_execution_output: None,
            }
        }

        Commands::Preset { preset, interval } => {
            res = DwExecuionResult {
                sucess: true,
                exit_code: 0,
                message: format!("Sucess"),
                sys_commando_execution_output: None,
            }
        }

        Commands::Next => {
            res = DwExecuionResult {
                sucess: true,
                exit_code: 0,
                message: format!("Sucess"),
                sys_commando_execution_output: None,
            }
        }

        Commands::Reset => {
            res = DwExecuionResult {
                sucess: true,
                exit_code: 0,
                message: format!("Sucess"),
                sys_commando_execution_output: None,
            }
        }

        Commands::Set { path } => res = dw_commands_operations::set_wallpaper(path),

        Commands::Off => {
            res = DwExecuionResult {
                sucess: true,
                exit_code: 0,
                message: format!("Sucess"),
                sys_commando_execution_output: None,
            }
        }

        Commands::On => {
            res = DwExecuionResult {
                sucess: true,
                exit_code: 0,
                message: format!("Sucess"),
                sys_commando_execution_output: None,
            }
        }
    };

    if !res.sucess {
        eprint!("{}", res.message);
        process::exit(res.exit_code as i32);
    }
}

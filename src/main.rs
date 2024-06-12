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

mod core;
mod models;
mod operations;

use crate::models::{Cli, Commands, DwOperationExecutionResult};
use crate::operations::{perform_init, set_wallpaper, show_config};
use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli: Cli = Cli::parse();
    let operation_res;

    match &cli.command {
        Commands::AddWallpaper { path: _ } => {
            operation_res = DwOperationExecutionResult {
                success: true,
                exit_code: 0,
                message: None,
            }
        }

        Commands::RemoveWallpaper { path: _ } => {
            operation_res = DwOperationExecutionResult {
                success: true,
                exit_code: 0,
                message: None,
            }
        }

        Commands::Preset {
            preset: _,
            interval: _,
        } => {
            operation_res = DwOperationExecutionResult {
                success: true,
                exit_code: 0,
                message: None,
            }
        }

        Commands::Next => {
            operation_res = DwOperationExecutionResult {
                success: true,
                exit_code: 0,
                message: None,
            }
        }

        Commands::Reset => {
            operation_res = DwOperationExecutionResult {
                success: true,
                exit_code: 0,
                message: None,
            }
        }

        Commands::SetWallpaper { path } => {
            operation_res = set_wallpaper(path);
        }

        Commands::Off => {
            operation_res = DwOperationExecutionResult {
                success: true,
                exit_code: 0,
                message: None,
            }
        }

        Commands::On => {
            operation_res = DwOperationExecutionResult {
                success: true,
                exit_code: 0,
                message: None,
            }
        }

        Commands::ShowConfig => {
            operation_res = show_config();
        }

        Commands::SetConfig => {
            operation_res = DwOperationExecutionResult {
                success: true,
                exit_code: 0,
                message: None,
            }
        }

        Commands::Init => {
            operation_res = perform_init();
        }
    };

    if operation_res.success {
        print!("{:?}", operation_res);
        return ExitCode::SUCCESS;
    } else {
        eprint!("{:?}", operation_res);
        return ExitCode::FAILURE;
    }
}

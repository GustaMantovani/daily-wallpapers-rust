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

// src/dw_core_functions.rs

use std::path::Path;
use std::process::Command;

// Our things 👥
use crate::dw_models::DwExecuionResult;

pub fn change_wallpaper(path: &str) -> DwExecuionResult {
    if !Path::new(path).exists() {
        return DwExecuionResult {
            sucess: false,
            exit_code: 1,
            message: format!("Path {path} not found"),
            command_execution_output: None,
        };
    }

    let command = format!(
        "gsettings set org.gnome.desktop.background picture-uri-dark {}",
        path
    );

    let output = Command::new("bash")
        .arg("-c")
        .arg(command.clone())
        .output()
        .expect(&format!("Failed to execute the process bash -c {command}"));

    if output.status.success() {
        return DwExecuionResult {
            sucess: true,
            exit_code: 0,
            message: format!("Sucess"),
            command_execution_output: Some(output),
        };
    } else {
        return DwExecuionResult {
            sucess: false,
            exit_code: 1,
            message: format!("Path {path} not found"),
            command_execution_output: None,
        };
    }
}

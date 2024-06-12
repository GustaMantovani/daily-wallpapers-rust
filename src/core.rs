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

// src/core.rs

use std::{path::Path, process::Command};

pub fn change_wallpaper(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Err("O caminho do arquivo não existe".into());
    }

    let command = format!(
        "gsettings set org.gnome.desktop.background picture-uri-dark {:?}",
        path
    );

    let command_execution_output = Command::new("sh")
        .args(["-c", &command])
        .output()
        .expect("failed to execute process");

    if command_execution_output.status.success() {
        Ok(())
    } else {
        Err("O comando para alterar o papel de parede falhou".into())
    }
}

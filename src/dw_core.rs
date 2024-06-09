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

// src/dw_core.rs
use std::path::Path;
use std::process::Command;

pub fn change_wallpaper(path: &String) {
    if !Path::new(path).exists() {
        eprintln!("O caminho especificado não existe: {}", path);
        return;
    }

    let command = "gsettings set org.gnome.desktop.background picture-uri-dark ".to_owned()
        + &path.to_owned();

    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Err: {}", stderr);
        //Exibir notificação de erro no gnome
    }
}

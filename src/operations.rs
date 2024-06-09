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

// Our things 👥
#[path = "./dw_core.rs"]
mod dw_core;

pub fn add_wallpaper(path: &str) {
    println!("Adding wallpaper: {}", path);
}

pub fn remove_wallpaper(path: &str) {
    println!("Removing wallpaper: {}", path);
}

pub fn set_preset(preset: &str, interval: Option<u64>) {
    println!("Setting preset: {} with interval: {:?}", preset, interval);
}

pub fn set_next_wallpaper() {
    println!("Setting the next wallpaper");
}

pub fn reset_wallpaper_cycle() {
    println!("Setting the first wallpapers");
}

pub fn set_wallpaper(path: &String) {
    println!("Setting {} as wallpaper", path);
    match dw_core::change_wallpaper(path) {
        Ok(_) => {},
        Err(err) => eprintln!("Erro: {}", err),
    }
}

pub fn disable_wallpapers() {
    println!("Disabling daily wallpapers");
}

pub fn enable_wallpapers() {
    println!("Enabling daily wallpapers");
}

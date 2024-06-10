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

pub fn add_wallpaper(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Adding wallpaper: {}", path);
    // Implementação da função...
    Ok(())
}

pub fn remove_wallpaper(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Removing wallpaper: {}", path);
    // Implementação da função...
    Ok(())
}

pub fn set_preset(preset: &str, interval: Option<u64>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting preset: {} with interval: {:?}", preset, interval);
    // Implementação da função...
    Ok(())
}

pub fn set_next_wallpaper() -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting the next wallpaper");
    // Implementação da função...
    Ok(())
}

pub fn reset_wallpaper_cycle() -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting the first wallpapers");
    // Implementação da função...
    Ok(())
}

pub fn set_wallpaper(path: &String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting {} as wallpaper", path);
    return dw_core::change_wallpaper(path);
}

pub fn disable_wallpapers() -> Result<(), Box<dyn std::error::Error>> {
    println!("Disabling daily wallpapers");
    // Implementação da função...
    Ok(())
}

pub fn enable_wallpapers() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enabling daily wallpapers");
    // Implementação da função...
    Ok(())
}

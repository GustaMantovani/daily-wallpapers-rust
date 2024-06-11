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

// src/dw_commands_operations.rs

// Our things 👥
use crate::dw_core_functions;
use crate::dw_models::DwExecuionResult;

pub fn set_wallpaper(path: &String) -> DwExecuionResult {
    println!("Setting {} as wallpaper", path);
    return dw_core_functions::change_wallpaper(path);
}

/*pub fn add_wallpaper(path: &str) -> CoreFunctionExecutionResult {
    println!("Adding wallpaper: {}", path);
    // Implementação da função...

}

pub fn remove_wallpaper(path: &str) -> CoreFunctionExecutionResult {
    println!("Removing wallpaper: {}", path);
    // Implementação da função...

}

pub fn set_preset(preset: &str, interval: Option<u64>) -> CoreFunctionExecutionResult {
    println!("Setting preset: {} with interval: {:?}", preset, interval);
    // Implementação da função...

}

pub fn set_next_wallpaper() -> CoreFunctionExecutionResult {
    println!("Setting the next wallpaper");
    // Implementação da função...

}

pub fn reset_wallpaper_cycle() -> CoreFunctionExecutionResult {
    println!("Setting the first wallpapers");
    // Implementação da função...

}

pub fn set_wallpaper(path: &String) -> CoreFunctionExecutionResult {
    println!("Setting {} as wallpaper", path);
    return dw_core::change_wallpaper(path);
}

pub fn disable_wallpapers() -> CoreFunctionExecutionResult {
    println!("Disabling daily wallpapers");
    // Implementação da função...

}

pub fn enable_wallpapers() -> CoreFunctionExecutionResult {
    println!("Enabling daily wallpapers");
    // Implementação da função...

}*/

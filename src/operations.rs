// Copyright 2023 Gustavo Mantovani
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

pub fn add_wallpaper(path: &str) {
    println!("Adding wallpaper: {}", path);
}

pub fn remove_wallpaper(path: &str) {
    println!("Removing wallpaper: {}", path);
}

pub fn set_preset(preset: &str, interval: Option<u64>) {
    println!("Setting preset: {} with interval: {:?}", preset, interval);
}

pub fn disable_wallpapers() {
    println!("Disabling daily wallpapers");
}

pub fn enable_wallpapers() {
    println!("Enabling daily wallpapers");
}

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

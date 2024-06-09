use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "dw", about = "Daily Wallpaper Manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Add a wallpaper or directory to wallpapers cycle")]
    AddWallpaper {
        #[arg()]
        path: String,
    },
    #[command(about = "Remove a wallpaper or directory from wallpapers cycle")]
    RemoveWallpaper {
        #[arg()]
        path: String,
    },
    #[command(about = "Set time preset to wallpaper change")]
    Preset {
        #[arg()]
        preset: String,
        #[arg(required_if_eq("preset", "by minutes"), required_if_eq("preset", "by hours"))]
        interval: Option<u64>,
    },
    #[command(about = "Disable daily wallpapers")]
    Off,
    #[command(about = "Enable daily wallpapers")]
    On,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::AddWallpaper { path } => {
            println!("Adding wallpaper: {}", path);
        }
        Commands::RemoveWallpaper { path } => {
            println!("Removing wallpaper: {}", path);
        }
        Commands::Preset { preset, interval } => {
            println!("Setting preset: {} with interval: {:?}", preset, interval);
        }
        Commands::Off => {
            println!("Disabling daily wallpapers");
        }
        Commands::On => {
            println!("Enabling daily wallpapers");
        }
    }
}

use std::path::Path;
use std::process::Command;

pub fn change_wallpaper(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(path).exists() {
        return Err("O caminho do arquivo não existe".into());
    }

    let command = format!(
        "gsettings set org.gnome.desktop.background picture-uri-dark {}",
        path
    );

    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|e| format!("Falha ao executar o processo: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err("O comando para alterar o papel de parede falhou".into())
    }
}

use std::fs;

fn main() {
    // Leia o Cargo.toml
    let cargo_toml_content = fs::read_to_string("Cargo.toml")
        .expect("Não foi possível ler o Cargo.toml");

    // Parse o conteúdo do Cargo.toml
    let cargo_toml: toml::Value = cargo_toml_content.parse()
        .expect("Não foi possível parsear o Cargo.toml");

    // Acesse a chave `[context]` e a variável `local`
    if let Some(context) = cargo_toml.get("context") {
        if let Some(local) = context.get("local") {
            let local_value = local.as_bool().unwrap_or(false);
            // Define a variável de ambiente que será usada em tempo de compilação
            println!("cargo:rustc-env=CONTEXT_LOCAL={}", local_value);
        }
    }
}

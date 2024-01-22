// vault.rs
use std::{fs, fs::File, io::Write, path::Path};

pub fn init_vault(path: String) {
    let default_vault_dir = crate::config::get_or_default("DEFAULT_VAULT_DIR", "docs");
    let obsidian_dir = crate::config::get_or_default("OBSIDIAN_DIR", ".obsidian");
    let defaults_dir = crate::config::get_or_default("DEFAULTS_DIR", "../defaults");

    let vault_name_from_path = path.split("/").last().unwrap();
    let vault_name = if vault_name_from_path == "" {
        "vault"
    } else {
        vault_name_from_path
    };

    let vault_path = format!("{}/{}", path, default_vault_dir);
    let vault_path = Path::new(&vault_path);

    // check if vault already exists
    if vault_path.exists() {
        return;
    }

    fs::create_dir_all(vault_path).unwrap();

    let config_path = format!("{}/{}", vault_path.to_str().unwrap(), obsidian_dir);
    let config_path = Path::new(&config_path);
    fs::create_dir_all(config_path).unwrap();

    let mut config = File::create(config_path.join("config")).unwrap();
    config
        .write_all(format!("{{\"vaults\":[{{\"path\":\"{}\"}}]}}", default_vault_dir).as_bytes())
        .unwrap();

    let index_md_path = vault_path.join("index.md");
    let mut index_md = File::create(&index_md_path).unwrap();
    index_md
        .write_all(format!("# {}\n\nWelcome to your new vault!", vault_name).as_bytes())
        .unwrap();

    // Load all JSON files from ../defaults
    let defaults_dir = Path::new(&defaults_dir);
    if let Ok(entries) = fs::read_dir(defaults_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let source_path = entry.path();
                let dest_path = config_path.join(source_path.file_name().unwrap());

                fs::copy(&source_path, &dest_path).unwrap();
            }
        }
    }
}

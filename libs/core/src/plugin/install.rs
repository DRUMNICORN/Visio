use crates_io_api::Crate;
use reqwest::blocking::Client;
use std::fs::{self, File};
use std::io::Write;
use toml::value::Table;

use std::path::PathBuf;

// Default Toml file
const DEFAULT_CARGO_TOML: &str = r#"[package]
name = "notime"
version = "0.1.0"
description = "Nodium Runtime"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
"#;

// Constants
const CRATES_IO_API_URL: &str = "https://crates.io/api/v1/crates";
const PATH_TO_EXTENSION_DIR: &str = "/home/roggen/Documents/GitHub/nodium/src/notime";

// in src/install_crate.rs
pub fn download_crate(krate: &Crate) -> Result<(), Box<dyn std::error::Error>> {
    // Read the current Cargo.toml file
    let cargo_toml_path = PathBuf::from(PATH_TO_EXTENSION_DIR).join("Cargo.toml");
    debug!("cargo_toml_path: {:?}", cargo_toml_path);

    // check if Cargo.toml exists
    if !cargo_toml_path.exists() {
        // create Cargo.toml
        let mut cargo_toml_file = File::create(cargo_toml_path.clone()).unwrap();
        cargo_toml_file
            .write_all(DEFAULT_CARGO_TOML.as_bytes())
            .unwrap();
        debug!("Created Cargo.toml");
    }
    let cargo_toml = fs::read_to_string(cargo_toml_path.clone()).unwrap();
    let mut cargo_toml_value: Table = toml::from_str(&cargo_toml).unwrap();
    debug!("cargo_toml_value: {:?}", cargo_toml_value);

    // Add the crate as a dependency
    let dependencies = cargo_toml_value
        .entry("dependencies")
        .or_insert(toml::Value::Table(Table::new()))
        .as_table_mut()
        .unwrap();
    dependencies.insert(
        krate.name.clone(),
        toml::Value::String(krate.max_version.clone()),
    );
    debug!("dependencies: {:?}", dependencies);

    // Write the updated Cargo.toml file
    let updated_cargo_toml = toml::to_string(&cargo_toml_value).unwrap();
    let mut cargo_toml_file = fs::File::create(cargo_toml_path).unwrap();
    match cargo_toml_file.write_all(updated_cargo_toml.as_bytes()) {
        Ok(_) => debug!("Updated Cargo.toml"),
        Err(e) => debug!("Error updating Cargo.toml: {}", e),
    }

    // build the crate
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("build");
    cmd.current_dir(PATH_TO_EXTENSION_DIR);
    match cmd.output() {
        Ok(_) => debug!("Built crate"),
        Err(e) => debug!("Error building crate: {}", e),
    }

    Ok(())
}

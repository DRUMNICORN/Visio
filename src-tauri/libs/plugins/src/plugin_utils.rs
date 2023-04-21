use log::debug;
use log::error;
use std::process::Command;

use crate::extract_crate_file;

pub async fn download(
    crate_name: &str,
    crate_version: &str,
    path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let download_url = format!(
        "https://crates.io/api/v1/crates/{}/{}/download",
        crate_name, crate_version
    );
    debug!("Downloading crate from {}", download_url);

    let crate_file_path = format!("{}-{}.crate", crate_name, crate_version);
    match extract_crate_file(&crate_file_path, &path.to_string()).await {
        Ok(_) => {
            debug!("Crate {} downloaded successfully", crate_name);
            Ok(())
        }
        Err(e) => {
            debug!("Crate {} failed to download", crate_name);
            Err(e)
        }
    }
}

pub async fn install(
    crate_name: &str,
    crate_version: &str,
    path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    debug!("Building crate {} to {}", crate_name, path);

    // let mut output_dir = format!("{}/{}", path, crate_name);
    // if lib == false {
    let output_dir = format!("{}/{}-{}", path, crate_name, crate_version);
    // }

    let manifest_path = format!("{}/Cargo.toml", output_dir);

    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg(manifest_path);
    let output = cmd.output().expect("Failed to execute cargo build command");
    match !output.status.success() {
        true => {
            debug!("Crate {} failed to build", crate_name);
            error!(
                "Cargo build output: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            Err("Failed to build crate".into())
        }
        false => {
            debug!("Crate {} built successfully", crate_name);
            Ok(())
        }
    }
}

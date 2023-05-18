use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error};
use std::process::Command;

pub async fn build(
    crate_name: &str,
    path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    debug!("Building crate {} to {}", crate_name, path);

    // let mut output_dir = format!("{}/{}", path, crate_name);
    // if lib == false {
    //     output_dir = format!("{}/{}-{}", path, crate_name, crate_version);
    // }
    
    let output_dir = path;

    let manifest_path = format!("{}/Cargo.toml", output_dir);

    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg(manifest_path);

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("/|\\- ")
            .template("{spinner:.green} {msg}"),
    );
    pb.set_message(&format!("Building crate {}", crate_name));
    let output = cmd.output().expect("Failed to execute cargo build command");
    pb.finish_and_clear();

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
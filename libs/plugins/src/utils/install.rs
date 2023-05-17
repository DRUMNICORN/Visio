use log::debug;
use log::error;
use std::process::Command;
pub async fn install(
    crate_name: &str,
    crate_version: &str,
    path: &str,
    lib: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    debug!("Building crate {} to {}", crate_name, path);

    let mut output_dir = format!("{}/{}", path, crate_name);
    if lib == false {
        output_dir = format!("{}/{}-{}", path, crate_name, crate_version);
    }

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

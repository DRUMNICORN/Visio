use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

use crate::extract_crate_file;

use std::fs;
use std::path::Path;
use log::{debug, error};

pub async fn rebuild(install_location: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let plugins_dir = Path::new(install_location);
    if !plugins_dir.exists() {
        debug!("Plugins directory does not exist");
        match fs::create_dir_all(&plugins_dir) {
            Ok(_) => {
                debug!("Plugins directory created successfully");
            }
            Err(e) => {
                error!("Error creating plugins directory: {}", e);
                return Err(e.into());
            }
        }
    }

    let folders = match fs::read_dir(&plugins_dir) {
        Ok(folders) => folders,
        Err(e) => {
            error!("Error reading plugins directory: {}", e);
            return Err(e.into());
        }
    };

    for entry in folders {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                error!("Error reading plugin directory: {}", e);
                continue;
            }
        };
        let path = entry.path();
        debug!("Plugin path: {:?}", path);
        if path.is_dir() {
            let plugin_name = path.file_name().unwrap().to_str().unwrap();
            let plugin_version = path.file_name().unwrap().to_str().unwrap();
            debug!(
                "Plugin name and version: {} {}",
                plugin_name, plugin_version
            );

            // Remove the "target" folder and Cargo.lock file for each plugin
            let plugin_target_dir = path.join("target");
            if plugin_target_dir.exists() {
                fs::remove_dir_all(&plugin_target_dir).unwrap_or_else(|e| {
                    error!("Error removing plugin target directory: {}", e);
                });
            }

            let plugin_cargo_lock = path.join("Cargo.lock");
            if plugin_cargo_lock.exists() {
                fs::remove_file(&plugin_cargo_lock).unwrap_or_else(|e| {
                    error!("Error removing plugin Cargo.lock file: {}", e);
                });
            }
        }
    }

    Ok(())
}

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
    lib: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    debug!("Building crate {} to {}", crate_name, path);

    let mut output_dir = format!("{}/{}", path, crate_name);
    if !lib {
        output_dir = format!("{}/{}-{}", path, crate_name, crate_version);
    }

    let manifest_path = format!("{}/Cargo.toml", output_dir);

    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg(manifest_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().expect("Failed to execute cargo build command");

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    let stdout_lines = stdout_reader.lines();
    let stderr_lines = stderr_reader.lines();

    tokio::spawn(async move {
        for line in stdout_lines {
            if let Ok(line) = line {
                error!("{}", line);
            }
        }
    });
    
    tokio::spawn(async move {
        for line in stderr_lines {
            if let Ok(line) = line {
                // check if in line is the keyword "error"
                if line.contains("error") {
                    error!("{}", line);                    
                }else {
                    debug!("{}", line);
                }
            }
        }
    });

    let status = child.wait().expect("Failed to wait for cargo build command");

    if status.success() {
        debug!("Crate {} built successfully", crate_name);
        Ok(())
    } else {
        debug!("Crate {} failed to build", crate_name);
        Err("Failed to build crate".into())
    }
}


pub fn create_plugins_directory(plugins_dir: &Path) -> Result<(), std::io::Error> {
    if !plugins_dir.exists() {
        debug!("Plugins directory does not exist");
        fs::create_dir_all(&plugins_dir)?;
        debug!("Plugins directory created successfully");
    }
    Ok(())
}
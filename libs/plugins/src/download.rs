use std::fs;
use std::io::Write;
use std::path::Path;
use tar::Archive;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn download_and_extract_crate(crate_name: &str, version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = format!("external_crates/{}-{}", crate_name, version);

    // Create the output directory if it doesn't exist
    fs::create_dir_all(&output_dir)?;

    // Download the crate from crates.io
    let download_url = format!("https://crates.io/api/v1/crates/{}/download", crate_name);
  
    let tarball_path = format!("{}-{}.crate", crate_name, version);
    let mut tarball = File::create(&tarball_path).await?;
    let response = reqwest::get(&download_url).await?;
    let bytes = response.bytes().await?;
    tarball.write_all(&bytes).await?;

    // Extract the downloaded tarball
    let mut archive = Archive::new(std::fs::File::open(&tarball_path)?);
    let dest = Path::new(&output_dir);
    archive.unpack(dest)?;

    // Cleanup the downloaded tarball
    fs::remove_file(&tarball_path)?;

    Ok(())
}

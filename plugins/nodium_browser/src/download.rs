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
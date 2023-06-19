use crate::utils::extract::extract_crate_file;

pub fn process_download_node(crate_name: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let crate_version = "latest";
    let path = "";

    let download_url = format!(
        "https://crates.io/api/v1/crates/{}/{}/download",
        crate_name, crate_version
    );
    println!("Downloading crate from {}", download_url);

    let crate_file_path = format!("{}-{}.crate", crate_name, crate_version);
    match extract_crate_file(&crate_file_path, &path.to_string()){
        Ok(_) => {
            println!("Crate {} downloaded successfully", crate_name);
            Ok(())
        }
        Err(e) => {
            println!("Error downloading crate {}: {}", crate_name, e);
            Err(e.into())
        }
    }
}
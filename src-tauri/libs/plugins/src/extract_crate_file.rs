use std::fs::File;
use std::io::{BufReader};
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;

pub async fn extract_crate_file<P: AsRef<Path>>(crate_file_path: P, destination: P) -> Result<(), Box<dyn std::error::Error>> {
    // Open the .crate file
    let crate_file = File::open(crate_file_path)?;
    let reader = BufReader::new(crate_file);

    // Create a GzDecoder to handle the Gzip compression
    let gz_decoder = GzDecoder::new(reader);

    // Create a tar Archive to handle the tar format
    let mut archive = Archive::new(gz_decoder);

    // Extract the contents of the .crate file to the destination directory
    archive.unpack(destination)?;

    Ok(())
}

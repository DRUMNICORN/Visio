use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;

pub fn extract_crate_file<P: AsRef<Path>>(
    crate_file_path: P,
    destination: P,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let crate_file = File::open(crate_file_path)?;
    let reader = BufReader::new(crate_file);

    let gz_decoder = GzDecoder::new(reader);
    let mut archive = Archive::new(gz_decoder);

    archive.unpack(destination)?;

    Ok(())
}

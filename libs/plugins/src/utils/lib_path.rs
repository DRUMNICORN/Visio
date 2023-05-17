use std::path::Path;

use log::error;

pub fn get_lib_path(install_location: &str, folder_name: String, crate_name: &str) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Send + Sync>> {
  let lib_path = Path::new(install_location)
      .join(folder_name)
      .join("target")
      .join("release")
      .join(if cfg!(windows) { "lib" } else { "" })
      .join(format!(
          "lib{}{}",
          crate_name,
          if cfg!(windows) {
              ".dll"
          } else if cfg!(unix) {
              ".so"
          } else {
              error!("Unsupported platform");
              return Err("Unsupported platform".into());
          }
      ));
  Ok(lib_path)
}
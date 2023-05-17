
pub fn extract_folder_name(is_local: bool, crate_name: &str, crate_version: &str) -> String {
  let folder_name = if is_local {
      crate_name.to_string()
  } else {
      format!("{}-{}", crate_name, crate_version)
  };
  folder_name
}

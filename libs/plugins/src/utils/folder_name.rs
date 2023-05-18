
pub fn extract_folder_name(is_local: bool, crate_name: &str) -> String {
  let folder_name = if is_local {
      crate_name.to_string()
  } else {
      format!("{}", crate_name)
  };
  folder_name
}

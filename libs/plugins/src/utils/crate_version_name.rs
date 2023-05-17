use log::debug;
use serde_json::Value;


pub fn extract_crate_version_name(payload: String) -> (String, String) {
  let data: Value = serde_json::from_str(&payload).unwrap();
  debug!("Handling event: {}", payload);
  debug!("Event data: {:?}", data);

  let crate_version = data
      .get("crate_version")
      .and_then(Value::as_str)
      .unwrap()
      .to_string();
  let crate_name = data
      .get("crate_name")
      .and_then(Value::as_str)
      .unwrap()
      .to_string();
  (crate_version, crate_name)
}

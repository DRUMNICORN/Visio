// main.rs
use reqwest::Error;
use serde_json::Value;

async fn fetch_crates() -> Result<Vec<CrateInfo>, Error> {
    let response = reqwest::get("https://crates.io/api/v1/crates")
        .await?
        .json::<Value>()
        .await?;

    let crates = response["crates"]
        .as_array()
        .expect("Failed to parse crates array")
        .iter()
        .map(|c| {
            CrateInfo::new(
                c["id"].as_str().unwrap().to_string(),
                c["name"].as_str().unwrap().to_string(),
                c["description"].as_str().unwrap_or("").to_string(),
                c["updated_at"].as_str().unwrap().to_string(),
                c["downloads"].as_u64().unwrap(),
            )
        })
        .collect::<Vec<CrateInfo>>();

    Ok(crates)
}

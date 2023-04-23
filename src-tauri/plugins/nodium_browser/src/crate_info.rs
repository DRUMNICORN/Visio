// main.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct CrateInfo {
    id: String,
    name: String,
    description: String,
    updated_at: String,
    downloads: u64,
}

impl CrateInfo {
    fn new(id: String, name: String, description: String, updated_at: String, downloads: u64) -> Self {
        Self {
            id,
            name,
            description,
            updated_at,
            downloads,
        }
    }
}

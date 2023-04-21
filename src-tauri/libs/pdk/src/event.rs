use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodiumEvent {
    pub name: String,
    pub payload: String,
} 

impl NodiumEvent {
    pub fn new(name: &str, payload: String) -> Self {
        NodiumEvent {
            name: name.to_string(),
            payload,
        }
    }
}
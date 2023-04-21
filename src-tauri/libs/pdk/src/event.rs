use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodiumEvent {
    pub name: String,
    pub payload: String,
} 


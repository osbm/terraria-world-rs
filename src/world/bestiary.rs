use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bestiary {
    pub kills: std::collections::HashMap<String, i32>,
    pub sightings: Vec<String>,
    pub chats: Vec<String>,
}

impl Bestiary {
    pub fn new(
        kills: std::collections::HashMap<String, i32>,
        sightings: Vec<String>,
        chats: Vec<String>,
    ) -> Self {
        Self {
            kills,
            sightings,
            chats,
        }
    }
} 
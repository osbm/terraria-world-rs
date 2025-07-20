use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bestiary {
    pub kills: Vec<(String, i32)>, // Preserve order as read from file
    pub kills_lookup: std::collections::HashMap<String, i32>, // For quick lookups
    pub sightings: Vec<String>,
    pub chats: Vec<String>,
}

impl Bestiary {
    pub fn new(
        kills: Vec<(String, i32)>, // Changed from HashMap to Vec to preserve order
        sightings: Vec<String>,
        chats: Vec<String>,
    ) -> Self {
        // Create lookup HashMap from the ordered Vec
        let kills_lookup = kills.iter().cloned().collect();

        Self {
            kills,
            kills_lookup,
            sightings,
            chats,
        }
    }

    /// Get a kill count for a specific entity
    pub fn get_kills(&self, entity: &str) -> i32 {
        self.kills_lookup.get(entity).copied().unwrap_or(0)
    }

    /// Add or update a kill count while preserving order
    pub fn add_kills(&mut self, entity: String, count: i32) {
        // Update lookup
        self.kills_lookup.insert(entity.clone(), count);

        // Update or add to ordered list
        if let Some(index) = self.kills.iter().position(|(e, _)| e == &entity) {
            self.kills[index] = (entity, count);
        } else {
            self.kills.push((entity, count));
        }
    }
}

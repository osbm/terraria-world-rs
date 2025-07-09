use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityType(pub i32);

impl EntityType {
    pub fn new(id: i32) -> Self {
        Self(id)
    }

    pub fn id(&self) -> i32 {
        self.0
    }
}

impl From<i32> for EntityType {
    fn from(value: i32) -> Self {
        Self(value)
    }
} 
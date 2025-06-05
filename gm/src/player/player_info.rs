use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerInfo {
    pub id: PlayerId,
    pub username: PlayerUsername,
    pub levels: PlayerLevelInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerLevelInfo {
    pub player_level: f32,
    pub to_next: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
pub struct PlayerUsername(pub String);

impl PlayerUsername {
    pub fn new(username: &str) -> Self {
        PlayerUsername(String::from(username))
    }
}

impl std::fmt::Display for PlayerUsername {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
pub struct PlayerId(Uuid);

impl PlayerId {
    pub fn new_id() -> Self {
        PlayerId(Uuid::new_v4())
    }

    pub fn get_id(&self) -> &Uuid {
        &self.0
    }
}

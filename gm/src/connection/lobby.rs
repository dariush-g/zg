use std::{
    collections::HashMap,
    net::{SocketAddr, TcpStream},
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::player::player_data::Player;

#[derive(Debug, Clone)]
pub struct Lobby {
    pub clients: Arc<Mutex<HashMap<Player, (TcpStream, SocketAddr)>>>,
    pub id: LobbyId,
}

impl Default for Lobby {
    fn default() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            id: LobbyId::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyId(Uuid);
impl LobbyId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

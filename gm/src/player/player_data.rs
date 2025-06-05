use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    player_info::{PlayerId, PlayerInfo, PlayerLevelInfo, PlayerUsername},
    player_stats::PlayerStats,
};

#[derive(Clone, Debug, Component, Serialize, Deserialize)]
pub struct Player {
    pub info: PlayerInfo,
    pub pos: PlayerPositioning,
    pub stats: PlayerStats,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at pos: ({:.2}, {:.2}, {:.2})",
            self.info.username, self.pos.loc.x, self.pos.loc.y, self.pos.loc.z
        )
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            info: PlayerInfo {
                id: PlayerId::new_id(),
                username: PlayerUsername::new("username"),
                levels: PlayerLevelInfo {
                    player_level: 0.,
                    to_next: 0.,
                },
            },
            pos: PlayerPositioning {
                loc: Vec3::ZERO,
                dir: Quat::default(),
                vel: Vec3::ZERO,
                grounded: false,
            },
            stats: PlayerStats::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerPositioning {
    pub loc: Vec3,
    pub dir: Quat,
    pub vel: Vec3,
    pub grounded: bool,
}

#[allow(unused)]
impl PlayerPositioning {
    pub fn new(loc: Vec3, dir: Quat) -> Self {
        Self {
            loc,
            dir,
            vel: Vec3::ZERO,
            grounded: false,
        }
    }

    pub fn get_loc(&self) -> Vec3 {
        self.loc
    }
    pub fn get_dir(&self) -> Quat {
        self.dir
    }
    pub fn set_dir(&mut self, dir: Quat) {
        self.dir = dir;
    }
    pub fn set_loc(&mut self, loc: Vec3) {
        self.loc = loc;
    }
    pub fn get_vel(&self) -> Vec3 {
        self.vel
    }
    pub fn set_vel(&mut self, vel: Vec3) {
        self.vel = vel;
    }
    pub fn is_grounded(&self) -> bool {
        self.grounded
    }
    pub fn set_grounded(&mut self, grounded: bool) {
        self.grounded = grounded;
    }
}

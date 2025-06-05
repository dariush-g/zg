use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerStats {
    pub health: Health,
    pub defense: Defense,
    pub stamina: Stamina,
    pub speed: Speed,
    pub vitality: Vitality,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            health: Health {
                max: 100.,
                current: 100.,
            },
            defense: Defense { defense: 0. },
            speed: Speed { speed: 10. },
            stamina: Stamina { stamina: 10. },
            vitality: Vitality { vitality: 10. },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn percent(&self) -> f32 {
        self.current / self.max
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Defense {
    pub defense: f32,
}

impl Defense {
    pub fn new(defense: f32) -> Self {
        Self { defense }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Speed {
    pub speed: f32,
}

impl Speed {
    pub fn new(x: f32) -> Self {
        Self { speed: x }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vitality {
    pub vitality: f32,
}

impl Vitality {
    pub fn new(x: f32) -> Self {
        Self { vitality: x }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stamina {
    pub stamina: f32,
}

impl Stamina {
    pub fn new(x: f32) -> Self {
        Self { stamina: x }
    }
}

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GamePlayerStats {
    pub turns_played: i32,
    pub survived: bool,
    pub fleet_generated: i32,
    pub fleet_lost: i32,
    pub fleet_reinforced: i32,
    pub largest_attack: i32,
    pub largest_loss: i32,
    pub largest_reinforcement: i32,
    pub planets_lost: i32,
    pub planets_conquered: i32,
    pub planets_defended: i32,
    pub planets_attacked: i32,
    pub num_fleet_lost: i32,
    pub num_fleet_reinforced: i32,
    pub num_fleet_generated: i32,
    pub total_troops_generated: i32,
}

impl Default for GamePlayerStats {
    fn default() -> Self {
        Self { turns_played: Default::default(), survived: Default::default(), fleet_generated: Default::default(), fleet_lost: Default::default(), fleet_reinforced: Default::default(), largest_attack: Default::default(), largest_loss: Default::default(), largest_reinforcement: Default::default(), planets_lost: Default::default(), planets_conquered: Default::default(), planets_defended: Default::default(), planets_attacked: Default::default(), num_fleet_lost: Default::default(), num_fleet_reinforced: Default::default(), num_fleet_generated: Default::default(), total_troops_generated: Default::default() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameError {
    pub error: String,
    pub blame_id: String,
}


impl Default for GameError {
    fn default() -> Self {
        Self { error: Default::default(), blame_id: Default::default() }
    }
}
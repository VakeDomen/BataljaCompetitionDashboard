use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GamePlayerStats {
    pub turnsPlayed: i32,
    pub winner: bool,
    pub fleetGenerated: i32,
    pub fleetLost: i32,
    pub fleetReinforced: i32,
    pub largestAttack: i32,
    pub largestLoss: i32,
    pub largestReinforcement: i32,
    pub planetsLost: i32,
    pub planetsConquered: i32,
    pub planetsDefended: i32,
    pub planetsAttacked: i32,
    pub numFleetLost: i32,
    pub numFleetReinforced: i32,
    pub numFleetGenerated: i32,
    pub totalTroopsGenerated: i32,
}


impl Default for GamePlayerStats {
    fn default() -> Self {
        Self { turnsPlayed: Default::default(), winner: Default::default(), fleetGenerated: Default::default(), fleetLost: Default::default(), fleetReinforced: Default::default(), largestAttack: Default::default(), largestLoss: Default::default(), largestReinforcement: Default::default(), planetsLost: Default::default(), planetsConquered: Default::default(), planetsDefended: Default::default(), planetsAttacked: Default::default(), numFleetLost: Default::default(), numFleetReinforced: Default::default(), numFleetGenerated: Default::default(), totalTroopsGenerated: Default::default() }
    }
}
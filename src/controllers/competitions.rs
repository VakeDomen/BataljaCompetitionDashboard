use crate::{models::errors::MatchMakerError, db::operations_competition::get_running_competitions};

use super::matchmaker_2v2::run_2v2_round;



pub fn run_competitions_round() -> Result<(), MatchMakerError> {
    let competitions = match get_running_competitions() {
        Ok(c) => c,
        Err(e) => return Err(MatchMakerError::DatabaseError(e)),
    };
    
    for competition in competitions.into_iter() {
        match competition.type_.as_str() {
            "2v2" => run_2v2_round(competition.id)?,
            _ => continue,
        }
    }
    Ok(())
}
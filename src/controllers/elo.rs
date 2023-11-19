use diesel::result::Error;

use crate::{models::game_2v2::{Game2v2, NewGame2v2}, db::operations_teams::{get_team_by_id, add_elo_change}};

const ELO_K_FAATOR: i32 = 16;

pub fn update_team_elo(games: Vec<Game2v2>) -> Result<(), Error> {
    for game in games.into_iter() {
        if let Err(e) = add_elo_change(game.team1_id, game.team1_elo) {
            return Err(e.into());
        } 
        if let Err(e) = add_elo_change(game.team2_id, game.team2_elo) {
            return Err(e.into());
        } 
    }
    Ok(())
}

pub fn calc_elo_changes(game: &mut NewGame2v2) -> Result<(), Error> {
    let team1 = match get_team_by_id(game.team1_id.clone()) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };
    let team2 = match get_team_by_id(game.team2_id.clone()) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    let result_team1 = if game.winner_id == game.team1_id { 1.0 } else { 0.0 };
    let result_team2 = 1.0 - result_team1; // Opposite of team1's result

    game.team1_elo = calculate_elo_change(team1.elo, team2.elo, result_team1);
    game.team2_elo = calculate_elo_change(team2.elo, team1.elo, result_team2);

    Ok(())
}

fn calculate_elo_change(player_elo: i32, opponent_elo: i32, result: f64) -> i32 {
    let expected_score = 1.0 / (1.0 + 10.0_f64.powf((opponent_elo - player_elo) as f64 / 400.0));
    (ELO_K_FAATOR as f64 * (result - expected_score)).round() as i32
}
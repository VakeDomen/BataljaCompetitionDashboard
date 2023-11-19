use std::collections::HashMap;

use actix_web::{HttpResponse, get, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::{
    controllers::jwt::exchange_token_for_user, 
    db::{
        operations_teams::get_team_by_id, 
        operations_game2v2::get_rounds_for_competition, 
        operations_competition::get_competition_by_id
    }, 
    models::game_2v2::Game2v2,
};

type RoundData = (
    i32,            // points gained/lost in the round
    String,         // bot 1 id
    String,         // bot 2 id
    Vec<String>     // vec of ids of matches
);

#[get("/competition/rounds/{team_id}")]
pub async fn competition_rounds(auth: BearerAuth, team_id: web::Path<String>) -> HttpResponse {
    let requesting_user = match exchange_token_for_user(auth) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish()
    };

    let team_id = team_id.into_inner();

    let team = match get_team_by_id(team_id.clone()) {
        Ok(t) => t,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    if requesting_user.id != team.owner && requesting_user.id != team.partner {
        return HttpResponse::Unauthorized().finish();
    }

    let competition = match get_competition_by_id(team.competition_id.clone()) {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let games = match get_rounds_for_competition(team.id, competition.id) {
        Ok(games) => games,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    HttpResponse::Ok().json(construct_output(games, team_id))
}


fn construct_output(games: Vec<Game2v2>, team_id: String) -> HashMap<i32, RoundData> {
    let mut hm: HashMap<i32, RoundData> = HashMap::new();
    for game in games.into_iter() {
        
        let bots = get_my_bots(&game, &team_id);
        let game_round = game.round.clone();
        let current_round_score = hm.entry(game_round).or_insert((0, bots.0, bots.1, Vec::new()));
        // log game id
        current_round_score.3.push(game.id.clone());
        

        if game.team1_id == game.team2_id {
            current_round_score.0 += 0;
        } else if game.team1_id == team_id {
            current_round_score.0 += game.team1_elo;
        } else {
            current_round_score.0 += game.team2_elo;
        }
    }
    hm
}

fn get_my_bots(game: &Game2v2, team_id: &str) -> (String, String) {
    if game.team1_id.eq(team_id) {
        (game.team1bot1_id.clone(), game.team1bot2_id.clone())
    } else {
        (game.team2bot1_id.clone(), game.team2bot2_id.clone())
    }
}
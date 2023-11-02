use std::collections::HashMap;

use actix_web::{HttpResponse, get, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::{
    controllers::jwt::exchange_token_for_user, 
    db::{
        operations_teams::get_team_by_id, 
        operations_bot::get_bots_by_team, operations_game2v2::get_games_by_bot_id
    }, models::game_2v2::Game2v2,
};

#[get("/bots/wr/{team_id}")]
pub async fn bots_win_rate(auth: BearerAuth, team_id: web::Path<String>) -> HttpResponse {
    let requesting_user = match exchange_token_for_user(auth) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish()
    };

    let team_id = team_id.into_inner();

    let team = match get_team_by_id(team_id) {
        Ok(t) => t,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    if requesting_user.id != team.owner && requesting_user.id != team.partner {
        return HttpResponse::Unauthorized().finish();
    }

    let bots = match get_bots_by_team(team.id) {
        Ok(bots) => bots,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let mut hm: HashMap<String, (f32, f32)> = HashMap::new();

    for bot in bots.into_iter() {
        let games_played = match get_games_by_bot_id(bot.id.clone()) {
            Ok(games) => games,
            Err(_) => return HttpResponse::InternalServerError().finish()
        };
        let data = calc_win_rate(bot.id.clone(), games_played);
        hm.insert(bot.id, data);
    }

    HttpResponse::Ok().json(hm)

}

fn calc_win_rate(bot_id: String, games: Vec<Game2v2>) -> (f32, f32) {
    let mut win_count = 0.;
    let mut survival_count = 0.;
    let mut game_count = 0.;

    for game in games.into_iter() {
        let is_bot_1 = game.team1bot1_id.eq(&bot_id);
        let is_bot_2 = game.team1bot2_id.eq(&bot_id);
        let is_bot_3 = game.team2bot1_id.eq(&bot_id);
        let is_bot_4 = game.team2bot2_id.eq(&bot_id);

        if is_bot_1 {
            game_count += 1.;
            if game.team1bot1_survived {
                survival_count += 1.;
            }
            if game.winner_id.eq(&game.team1_id) {
                win_count += 1.;
            }
        }

        if is_bot_2 {
            game_count += 1.;
            if game.team1bot2_survived {
                survival_count += 1.;
            }
            if game.winner_id.eq(&game.team1_id) {
                win_count += 1.;
            }
        }

        if is_bot_3 {
            game_count += 1.;
            if game.team2bot1_survived {
                survival_count += 1.;
            }
            if game.winner_id.eq(&game.team2_id) {
                win_count += 1.;
            }
        }

        if is_bot_4 {
            game_count += 1.;
            if game.team2bot2_survived {
                survival_count += 1.;
            }
            if game.winner_id.eq(&game.team2_id) {
                win_count += 1.;
            }
        }
    }
    (
        win_count / game_count,
        survival_count / game_count
    )
}
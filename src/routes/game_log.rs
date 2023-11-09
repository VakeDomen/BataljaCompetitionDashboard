use std::fs;

use actix_web::{HttpResponse, get, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Serialize;
use crate::{
    db::{
        operations_game2v2::get_game_by_id, 
        operations_teams::get_team_by_student_for_competition
    }, 
    controllers::jwt::exchange_token_for_user
};

#[derive(Debug, Serialize)]
struct GameLogResponse {
    log_file_contents: String,
}

#[get("/game/log/{id}")]
pub async fn get_game_log(auth: Option<BearerAuth>, id: web::Path<String>) -> HttpResponse {
    let game = match get_game_by_id(id.clone()) {
        Ok(game) => game,
        Err(_) => return HttpResponse::NotFound().finish()
    };

    

    if !game.public {
        let auth_token = match auth {
            Some(token) => token,
            None => return HttpResponse::Forbidden().finish(),
        };
        let requesting_user_option = exchange_token_for_user(auth_token);
        let requesting_user = match requesting_user_option {
            Some(u) => u,
            None => return HttpResponse::Forbidden().finish(),
        };
        let team = match get_team_by_student_for_competition(requesting_user, game.competition_id.clone()) {
            Ok(t) => t,
            Err(_) => return HttpResponse::Unauthorized().finish(),
        };
        if !team.id.eq(&game.team1_id) && !team.id.eq(&game.team2_id) {
            return HttpResponse::Forbidden().finish();
        }
    }
    
    let log_file_path = game.log_file_path;

    // Load the log file contents here
    let log_file_contents = match fs::read_to_string(log_file_path) {
        Ok(log) => log,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Create a response struct with the log file contents
    let response = GameLogResponse {
        log_file_contents,
    };

    // Serialize the response struct to JSON
    let json_response = serde_json::to_string(&response).unwrap();

    // Return the JSON response with a 200 OK status
    HttpResponse::Ok()
        .content_type("application/json")
        .body(json_response)
}
use actix_web::{HttpResponse, post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::controllers::jwt::exchange_token_for_user;
use crate::db::operations_game2v2::{get_game_by_id, game_set_public};
use crate::db::operations_teams::get_team_by_student_for_competition;
use crate::models::user::Role;


#[post("/game/public/{game_id}")]
pub async fn game_toggle_public(auth: BearerAuth, game_id: web::Path<String>) -> HttpResponse {
    let user = match exchange_token_for_user(auth) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish()
    };
    let role = user.role.clone();
    let game = match get_game_by_id(game_id.clone()) {
        Ok(game) => game,
        Err(_) => return HttpResponse::NotFound().finish()
    };

    // does team exist
    let team = match get_team_by_student_for_competition(user, game.competition_id.clone()) {
        Ok(t) => t,
        Err(_) => return HttpResponse::Forbidden().finish(),
    };

    if 
        !game.team1_id.eq(&team.id) && 
        !game.team2_id.eq(&team.id) &&
        role != Role::Admin
        
    {
        return HttpResponse::Forbidden().finish();
    }

    let new_state = !game.public;

    match game_set_public(game.id, new_state) {
        Ok(_) =>  HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }

}
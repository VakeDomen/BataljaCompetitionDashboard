use actix_web::{HttpResponse, get, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::{models::game_2v2::PublicGame2v2, db::{operations_game2v2::get_game_by_id, operations_teams::get_team_by_student_for_competition}, controllers::jwt::exchange_token_for_user};

#[get("/game/{game_id}")]
pub async fn game_id(auth: Option<BearerAuth>, game_id: web::Path<String>) -> HttpResponse {
    let game = match get_game_by_id(game_id.into_inner()) {
        Ok(game) => game,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
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

    HttpResponse::Ok().json(PublicGame2v2::from(game))
}
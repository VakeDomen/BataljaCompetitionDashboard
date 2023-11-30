use actix_web::{HttpResponse, get};
use crate::{models::game_2v2::PublicGame2v2, db::operations_game2v2::get_public_games};

#[get("/game/public")]
pub async fn game_get_public() -> HttpResponse {
    let games = match get_public_games() {
        Ok(games) => games,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };
    
    HttpResponse::Ok().json(games
        .into_iter()
        .map(PublicGame2v2::from)
        .collect::<Vec<PublicGame2v2>>())
}
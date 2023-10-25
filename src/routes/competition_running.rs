use actix_web::{HttpResponse, get};
use crate::db::operations_competition::get_running_competitions;
use crate::models::competition::PublicCompetition;

#[get("/competition/running")]
pub async fn competition_running() -> HttpResponse {
    match get_running_competitions() {
        Ok(competitions) => HttpResponse::Ok().json(
            competitions
                .into_iter()
                .map(PublicCompetition::from)
                .collect::<Vec<PublicCompetition>>()
        ),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string())
    }
}
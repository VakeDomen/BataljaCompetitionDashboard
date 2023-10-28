use actix_web::{HttpResponse, get, web};
use crate::db::operations_competition::get_competition_by_id;
use crate::models::competition::PublicCompetition;

#[get("/competition/{comp_id}")]
pub async fn competition_id(comp_id: web::Path<String>) -> HttpResponse {
    match get_competition_by_id(comp_id.into_inner()) {
        Ok(competition) => HttpResponse::Ok().json(PublicCompetition::from(competition)),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string())
    }
}
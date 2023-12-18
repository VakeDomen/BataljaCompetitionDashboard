use actix_web::{HttpResponse, get};
use crate::controllers::competitions::run_competitions_round;

#[get("/mm/test")]
pub async fn mmt() -> HttpResponse {
    match run_competitions_round() {
        Ok(u) => HttpResponse::Ok().json(u),
        Err(e) => HttpResponse::Unauthorized().body(e.to_string())
    }
}
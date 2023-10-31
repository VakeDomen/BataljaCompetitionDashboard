use actix_web::{HttpResponse, get};
use crate::controllers::matchmaker_2v2::run_2v2_round;

#[get("/mm/test")]
pub async fn mmt() -> HttpResponse {
    match run_2v2_round("6a84a31d-7c20-4e1c-9787-a8d8f92ea0a9".to_string()) {
        Ok(u) => HttpResponse::Ok().json(u),
        Err(e) => HttpResponse::Unauthorized().body(e.to_string())
    }
}
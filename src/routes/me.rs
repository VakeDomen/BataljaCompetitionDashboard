use actix_web::{HttpResponse, get};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::{controllers::jwt::exchange_token_for_user, models::user::PublicUser};

#[get("/me")]
pub async fn me(auth: BearerAuth) -> HttpResponse {
    match exchange_token_for_user(auth) {
        Some(u) => HttpResponse::Ok().json(PublicUser::from(u)),
        None => HttpResponse::Unauthorized().finish()
    }
}
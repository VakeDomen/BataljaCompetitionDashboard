use actix_web::{HttpResponse, post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::controllers::jwt::exchange_token_for_user;
use crate::db::operations_competition::insert_competition;
use crate::models::competition::{NewCompetition, PublicCompetition};
use crate::models::user::Role;

#[post("/competition")]
pub async fn competition_create(auth: BearerAuth, body: web::Json<NewCompetition>) -> HttpResponse {
    let requesting_user = match exchange_token_for_user(auth) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish()
    };

    if Role::Admin != requesting_user.role {
        return HttpResponse::Forbidden().finish();
    }

    match insert_competition(body.into_inner()) {
        Ok(c) => HttpResponse::Ok().json(PublicCompetition::from(c)),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string())
    }
}
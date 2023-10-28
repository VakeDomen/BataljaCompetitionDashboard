use actix_web::{HttpResponse, get, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::{controllers::jwt::exchange_token_for_user, models::user::PublicUser, db::operations_users::get_user_by_id};

#[get("/user/{user_id}")]
pub async fn user_id(auth: BearerAuth, user_id: web::Path<String>) -> HttpResponse {
    if let None = exchange_token_for_user(auth) {
        return HttpResponse::Unauthorized().finish()
    }

    match get_user_by_id(user_id.into_inner()) {
        Ok(u) => HttpResponse::Ok().json(PublicUser::from(u)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
use actix_web::{HttpResponse, post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::controllers::jwt::exchange_token_for_user;
use crate::db::operations_competition::get_competition_by_id;
use crate::db::operations_teams::create_team;
use crate::models::team::{NewTeam, PublicTeam};

#[post("/team")]
pub async fn team_create(auth: BearerAuth, body: web::Json<NewTeam>) -> HttpResponse {
    let user = match exchange_token_for_user(auth) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish()
    };

    let new_team = body.into_inner();

    if !user.id.eq(&new_team.owner) {
        return HttpResponse::Forbidden().finish();
    }


    // does competition exist
    if let Err(_) = get_competition_by_id(new_team.competition_id.clone()) {
        return HttpResponse::BadRequest().finish();
    }

    match create_team(new_team) {
        Ok(c) => HttpResponse::Ok().json(PublicTeam::from(c)),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string())
    }
}
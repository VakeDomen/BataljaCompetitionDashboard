use actix_web::{HttpResponse, post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;
use crate::controllers::jwt::exchange_token_for_user;
use crate::db::operations_teams::{get_team_by_student_for_competition, set_team_name};
use crate::models::team::PublicTeam;


#[derive(Debug, Deserialize)]
pub struct ChangeNameData {
    pub name: String,
    pub competition_id: String,
}

#[post("/team/name")]
pub async fn team_name_change(auth: BearerAuth, body: web::Json<ChangeNameData>) -> HttpResponse {
    let user = match exchange_token_for_user(auth) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish()
    };

    let change_name_data = body.into_inner();

    // does team exist
    let team = match get_team_by_student_for_competition(user, change_name_data.competition_id) {
        Ok(t) => t,
        Err(_) => return HttpResponse::Forbidden().finish(),
    };

    match set_team_name(&team, change_name_data.name) {
        Ok(t) =>  HttpResponse::Ok().json(PublicTeam::from(t)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }

}
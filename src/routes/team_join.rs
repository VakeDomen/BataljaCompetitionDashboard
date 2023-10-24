use actix_web::{HttpResponse, post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;
use crate::controllers::jwt::exchange_token_for_user;
use crate::db::operations_teams::{get_team_by_id, join_team};

#[derive(Debug, Deserialize)]
pub struct JoinTeamData {
    pub team_id: String,
}

#[post("/team/join")]
pub async fn team_join(auth: BearerAuth, body: web::Json<JoinTeamData>) -> HttpResponse {
    let user = match exchange_token_for_user(auth) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish()
    };

    let join_team_data = body.into_inner();


    // does team exist
    let team = match get_team_by_id(join_team_data.team_id.clone()) {
        Ok(t) => t,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    // can't join own team
    if team.owner == user.id {
        return HttpResponse::BadRequest().finish();
    }

    // can't join full team
    if !team.partner.eq("") {
        return HttpResponse::BadRequest().finish();
    }

    match join_team(team, user) {
        Ok(_) =>  HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }

}
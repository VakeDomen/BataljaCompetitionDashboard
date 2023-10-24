use actix_web::{HttpResponse, post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;
use crate::controllers::jwt::exchange_token_for_user;
use crate::db::operations_teams::{get_team_by_id, leave_team};

#[derive(Debug, Deserialize)]
pub struct LeaveTeamData {
    pub team_id: String,
}

#[post("/team/leave")]
pub async fn team_leave(auth: BearerAuth, body: web::Json<LeaveTeamData>) -> HttpResponse {
    let user = match exchange_token_for_user(auth) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish()
    };

    let leave_team_data = body.into_inner();


    // does team exist
    let team = match get_team_by_id(leave_team_data.team_id.clone()) {
        Ok(t) => t,
        Err(_) => return HttpResponse::Forbidden().finish(),
    };

    // can't leave own team
    if team.owner == user.id {
        return HttpResponse::Forbidden().finish();
    }

    // can't leave if not partner
    if !team.partner.eq(&user.id) {
        return HttpResponse::BadRequest().finish();
    }

    match leave_team(team, user) {
        Ok(_) =>  HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }

}
use actix_web::{HttpResponse, post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;
use crate::controllers::jwt::exchange_token_for_user;
use crate::db::operations_teams::{get_team_by_id, kick_partner};

#[derive(Debug, Deserialize)]
pub struct KickPartnerData {
    pub team_id: String,
}

#[post("/team/kick")]
pub async fn team_kick(auth: BearerAuth, body: web::Json<KickPartnerData>) -> HttpResponse {
    let user = match exchange_token_for_user(auth) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish()
    };

    let kick_team_data = body.into_inner();


    // does team exist
    let team = match get_team_by_id(kick_team_data.team_id.clone()) {
        Ok(t) => t,
        Err(_) => return HttpResponse::Forbidden().finish(),
    };

    // can't kick if not own team
    if team.owner != user.id {
        return HttpResponse::Forbidden().finish();
    }

    match kick_partner(team, user) {
        Ok(_) =>  HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }

}
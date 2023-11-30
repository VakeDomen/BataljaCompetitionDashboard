use actix_web::{HttpResponse, get, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::{
    controllers::jwt::exchange_token_for_user, 
    models::{bot::PublicBot, user::Role}, 
    db::{
        operations_teams::get_team_by_id, 
        operations_bot::get_bots_by_team
    },
};

#[get("/team/bots/{team_id}")]
pub async fn team_bots(auth: BearerAuth, team_id: web::Path<String>) -> HttpResponse {
    let requesting_user = match exchange_token_for_user(auth) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish()
    };

    let team_id = team_id.into_inner();

    let team = match get_team_by_id(team_id) {
        Ok(t) => t,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    if 
        requesting_user.id != team.owner && 
        requesting_user.id != team.partner && 
        requesting_user.role != Role::Admin 
    {
        return HttpResponse::Unauthorized().finish();
    }

    match get_bots_by_team(team.id) {
        Ok(bots) => HttpResponse::Ok().json(
            bots
                .into_iter()
                .map(PublicBot::from)
                .collect::<Vec<PublicBot>>()
        ),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}
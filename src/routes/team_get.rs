use actix_web::{HttpResponse, get};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::{
    controllers::jwt::exchange_token_for_user, 
    models::team::PublicTeam, 
    db::operations_teams::get_team_by_student,
};

#[get("/team")]
pub async fn team_get(auth: BearerAuth) -> HttpResponse {
    let requesting_user = match exchange_token_for_user(auth) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish()
    };

    match get_team_by_student(requesting_user) {
        Ok(teams) => HttpResponse::Ok().json(
            teams
                .into_iter()
                .map(PublicTeam::from)
                .collect::<Vec<PublicTeam>>()
        ),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}
use actix_web::{HttpResponse, get};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::controllers::jwt::exchange_token_for_user;
use crate::db::operations_competition::get_competitions_by_ids;
use crate::db::operations_teams::get_team_by_student;
use crate::models::competition::PublicCompetition;

#[get("/competition/attended")]
pub async fn competition_attended(auth: BearerAuth) -> HttpResponse {
    let requesting_user = match exchange_token_for_user(auth) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish()
    };


    let teams = match get_team_by_student(requesting_user) {
        Ok(teams) => teams,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let competition_ids = teams
        .into_iter()
        .map(|t| t.competition_id)
        .collect::<Vec<String>>();

    
    match get_competitions_by_ids(competition_ids) {
        Ok(competitions) => HttpResponse::Ok().json(
            competitions
                .into_iter()
                .map(PublicCompetition::from)
                .collect::<Vec<PublicCompetition>>()
        ),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string())
    }
}

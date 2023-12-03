use actix_web::{HttpResponse, get, web};
use crate::{
    models::team::PublicTeam, 
    db::operations_teams::get_team_by_id,
};

#[get("/team/{id}")]
pub async fn team_id(id: web::Path<String>) -> HttpResponse {
    match get_team_by_id(id.into_inner()) {
        Ok(team) => HttpResponse::Ok().json(PublicTeam::from(team)),
        Err(_) => HttpResponse::Ok().finish()
    }
}
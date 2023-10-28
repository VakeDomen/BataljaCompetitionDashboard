use actix_web::{HttpResponse, post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Deserialize;
use crate::controllers::jwt::exchange_token_for_user;
use crate::db::operations_bot::get_bot_by_id_and_team;
use crate::db::operations_teams::{get_team_by_student_for_competition, set_team_bot};
use crate::models::team::BotSelector;


#[derive(Debug, Deserialize)]
pub struct ChangeBotData {
    pub competition_id: String,
    pub bot: BotSelector,
    pub bot_id: String,
}

#[post("/team/bot")]
pub async fn team_bot_change(auth: BearerAuth, body: web::Json<ChangeBotData>) -> HttpResponse {
    let user = match exchange_token_for_user(auth) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish()
    };

    let change_bot_data = body.into_inner();

    // does team exist
    let team = match get_team_by_student_for_competition(user, change_bot_data.competition_id) {
        Ok(t) => t,
        Err(_) => return HttpResponse::Forbidden().finish(),
    };

    // does bot exist?
    let bot = match get_bot_by_id_and_team(change_bot_data.bot_id, team.id.clone()) {
        Ok(b) => b,
        Err(_) => return HttpResponse::Forbidden().finish(),
    };

    match set_team_bot(&team, change_bot_data.bot, bot.id) {
        Ok(_) =>  HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }

}
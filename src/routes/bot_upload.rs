use std::{path::Path, fs};
use actix_multipart::form::{tempfile::TempFile, MultipartForm, text::Text};
use actix_web::{HttpResponse, post};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Local, Timelike, Datelike};
use zip::ZipArchive;
use crate::{controllers::jwt::exchange_token_for_user, models::{bot::{NewBot, PublicBot}, team::BotSelector}, db::{operations_teams::{get_team_by_id, set_team_bot}, operations_bot::insert_bot}};

#[derive(MultipartForm)]
pub struct BotUploadData {
    team_id: Text<String>,
    file: Option<TempFile>,
}

#[post("/bot/upload")]
pub async fn bot_upload(auth: BearerAuth, payload: MultipartForm<BotUploadData>) -> HttpResponse {
    let requesting_user = match exchange_token_for_user(auth) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish()
    };
    let bot_file_data = payload.into_inner();


    // get the uploader's alleged team
    let team = match get_team_by_id(bot_file_data.team_id.0) {
        Ok(t) => t,
        Err(_) => return HttpResponse::BadRequest().body("Team does not exist"),
    };

    // is uploader part of the team
    if team.owner != requesting_user.id && team.partner != requesting_user.id {
        return HttpResponse::Forbidden().finish();
    }

    // zip correctly uploaded?
    let bot_file = match bot_file_data.file {
        Some(f) => f,
        None => return HttpResponse::BadRequest().body("Can't extract zip file."),
    };

    // is it a zip?
    if ZipArchive::new(&bot_file.file).is_err() {
        return HttpResponse::BadRequest().body("Uploaded file is not a valid ZIP file");
    }

    
    let filename = match &bot_file.file_name {
        Some(name) => name.to_string(),
        None => "EpicBot.zip".to_string(), // Default name if filename is not provided
    };


    let now = Local::now();
    let time = format!(
        "{:04}-{:02}-{:02}-{:02}-{:02}-{:02}", 
        now.year(), 
        now.month(), 
        now.day(), 
        now.hour(), 
        now.minute(), 
        now.second()
    );
    let save_directory = Path::new("./resources/uploads")
        .join(team.competition_id.clone())
        .join(time);

    if let Err(_) = fs::create_dir_all(&save_directory) {
        return HttpResponse::InternalServerError().body("Failed to create directory");
    }

    let save_path = save_directory.join(filename);
    
    let bot = NewBot { 
        team_id: team.id.clone(),
        source_path: save_path.to_string_lossy().to_string(), 
    };

    let bot = match insert_bot(bot) {
        Ok(b) => b,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    if team.bot1.eq("") {
        if let Err(_) = set_team_bot(&team, BotSelector::First, bot.id.clone()) {
            return HttpResponse::InternalServerError().finish();
        } 
    }

    if team.bot2.eq("") {
        if let Err(_) = set_team_bot(&team, BotSelector::Second, bot.id.clone()) {
            return HttpResponse::InternalServerError().finish();
        } 
    }

    match bot_file.file.persist(save_path) {
        Ok(_) => HttpResponse::Ok().json(PublicBot::from(bot)),
        Err(_) => HttpResponse::InternalServerError().body("Failed to save file"),
    }
}

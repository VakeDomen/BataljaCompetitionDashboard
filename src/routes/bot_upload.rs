use std::{path::Path, fs};
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{HttpResponse, post};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Local, Timelike, Datelike};
use crate::controllers::jwt::exchange_token_for_user;

#[derive(MultipartForm)]
pub struct BotUploadData {
    file: Option<TempFile>
}

#[post("/file/upload")]
pub async fn bot_upload(auth: BearerAuth, payload: MultipartForm<BotUploadData>) -> HttpResponse {
    let requesting_user = match exchange_token_for_user(auth) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().finish()
    };

    let bot_file_data = payload.into_inner();
    let bot_file = match bot_file_data.file {
        Some(f) => f,
        None => return HttpResponse::BadRequest().finish(),
    };


    let filename = match &bot_file.file_name {
        Some(name) => name.to_string(),
        None => "uploaded.zip".to_string(), // Default name if filename is not provided
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
    let save_directory = Path::new("./resources/uploads").join(time);
    if let Err(_) = fs::create_dir_all(&save_directory) {
        return HttpResponse::InternalServerError().body("Failed to create directory");
    }

    let save_path = save_directory.join(filename);
    
    match bot_file.file.persist(save_path) {
        Ok(_) => HttpResponse::Ok().body("File uploaded successfully!"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to save file"),
    }
}

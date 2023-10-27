use std::env;

use actix_cors::Cors;
use actix_web::HttpServer;
use actix_web_httpauth::extractors::bearer::Config;
use dotenv::dotenv;
use actix_web::{App, web, http, middleware::Logger};

use crate::routes::{
    login::login, 
    competition_create::competition_create, 
    team_create::team_create, 
    team_join::team_join, 
    team_leave::team_leave, 
    team_kick::team_kick, 
    bot_upload::bot_upload, 
    competition_running::competition_running, 
    me::me, 
    team_get::team_get, 
    team_bots::team_bots, 
    competition_attended::competition_attended,
};

mod routes;
mod controllers;
mod db;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    println!("[SETUP] Setting up environment.");
    let (port, url) = setup_env();
   
    // setup Http server
    let mut server = HttpServer::new(move || {
        // setup CORS

        let cors = match &url {
            Some(u) => Cors::default()
            .allowed_origin(u.as_str())
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600),
            None => Cors::permissive(),   
        };
        // setup routes
        App::new()
            // .wrap(Logger::default())
            .wrap(Logger::new("TIME: %T s | FROM: %a | RESP: %s | %r %{User-Agent}i (msg size in byted: %b)"))
            .wrap(cors)
            .app_data(Config::default())
            .service(
                web::scope("/api")
                .service(me)
                .service(login)
                .service(competition_create)
                .service(team_create)
                .service(team_join)
                .service(team_leave)
                .service(team_kick)
                .service(team_bots)
                .service(team_get)
                .service(bot_upload)
                .service(competition_running)
                .service(competition_attended)
            )
            
    });
    println!("[SETUP] Running server on 0.0.0.0:{}", port);
    server = server.bind(("0.0.0.0", port)).unwrap();
    server.run().await
}

fn setup_env() -> (u16, Option<String>) {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let port = env::var("PORT").expect("$PORT is not set").parse::<u16>().unwrap();
    let url = env::var("URL").ok();
    (port, url)
}
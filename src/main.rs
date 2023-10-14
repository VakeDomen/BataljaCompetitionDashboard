use std::env;

use actix_cors::Cors;
use actix_web::HttpServer;
use actix_web_httpauth::extractors::bearer::Config;
use dotenv::dotenv;
use actix_web::{App, web, http, middleware::Logger};

use crate::routes::login::login;

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
                .service(login)
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
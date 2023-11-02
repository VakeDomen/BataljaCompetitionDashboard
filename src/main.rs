use std::{env, thread};

use actix_cors::Cors;
use actix_web::HttpServer;
use actix_web_httpauth::extractors::bearer::Config;
use controllers::{matchmaker_2v2::run_2v2_round, competitions::run_competitions_round};
use dotenv::dotenv;
use actix_web::{App, web, http, middleware::Logger};
use tokio_cron_scheduler::{JobScheduler, Job};

use crate::routes::{
    login::login, 
    competition_create::competition_create, 
    team_create::team_create, 
    team_join::team_join, 
    team_leave::team_leave, 
    team_kick::team_kick, 
    bot_upload::bot_upload, 
    competition_running::competition_running, 
    user_me::user_me, 
    team_get::team_get, 
    team_bots::team_bots, 
    competition_attended::competition_attended,
    competition_id::competition_id, 
    user_id::user_id, 
    team_disband::team_disband, 
    team_bot_change::team_bot_change, 
    matchmaking_test::mmt, 
    bot_win_rates::bots_win_rate, competition_rounds::competition_rounds,
};

mod routes;
mod controllers;
mod db;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    println!("[SETUP] Setting up environment.");
    let (port, url) = setup_env();
   
    thread::spawn(|| {
        run_cron();
    });

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
                .service(user_me)
                .service(user_id)
                .service(login)
                .service(competition_create)
                .service(team_create)
                .service(team_disband)
                .service(team_join)
                .service(team_leave)
                .service(team_kick)
                .service(team_bots)
                .service(team_bot_change)
                .service(team_get)
                .service(bot_upload)
                .service(bots_win_rate)
                .service(competition_running)
                .service(competition_attended)
                .service(competition_id)
                .service(competition_rounds)
                .service(mmt)
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

/// Schedules and runs a cron job to execute the `run_competitions_round` function every hour.
///
/// This function sets up a cron job using the `JobScheduler` library. The cron job is scheduled to
/// run at the start of every hour, every day, and it calls the `run_competitions_round` function.
/// If there's any error while running the `run_competitions_round` function, the error is printed to the console.
///
/// Additionally, a shutdown handler is set up for the scheduler. This handler prints a shutdown message
/// when the scheduler is shutting down.
///
/// # Panics
///
/// This function will panic if there's an error setting up the cron job or starting the scheduler.
///
#[tokio::main]
async fn run_cron() {
    let mut sched = JobScheduler::new();
    match sched.add(Job::new_async("0 0 * * * * *", move |_, _|  Box::pin(async { 
        if let Err(e) = run_competitions_round() {
            println!("Error on running round: {:?}", e)
        }
    })).unwrap()) {
        Ok(c) => println!("Started cron!: {:?}", c),
        Err(e) => println!("Something went wrong scheduling CRON: {:?}", e)
    };

    // set shudown handler
    match sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
          println!("Shut down done");
        })
    })) {
        Ok(c) => println!("Shutdown handler set for cron!: {:?}", c),
        Err(e) => println!("Something went wrong setting shutdown handler for CRON: {:?}", e)
    };

    // start cron
    if let Err(e) = sched.start().await {
        println!("Error on scheduler {:?}", e);
    }
}

use diesel::result::Error;
use diesel::{prelude::*, insert_into};
use crate::db::schema::bots::dsl::*;
use crate::models::bot::{SqlBot, Bot, NewBot};
use super::operations_db::establish_connection;


pub fn insert_bot(bot: NewBot) ->  Result<Bot, Error> {
    let new_bot = SqlBot::from(bot);
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    let _ = insert_into(bots)
        .values(&new_bot)
        .execute(&mut conn)?;
    Ok(Bot::from(new_bot))
}

pub fn get_bot_by_id(uid: String) -> Result<Bot, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    match bots
        .filter(id.eq(uid))
        .first::<SqlBot>(&mut conn) {
            Ok(u) => Ok(Bot::from(u)),
            Err(e) => Err(e)
    }
}

pub fn get_bot_by_id_and_team(bot_id: String, tid: String) -> Result<Bot, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    match bots
        .filter(id.eq(bot_id).and(team_id.eq(tid)))
        .first::<SqlBot>(&mut conn) {
            Ok(u) => Ok(Bot::from(u)),
            Err(e) => Err(e)
    }
}


pub fn get_bots_by_team(tid: String) -> Result<Vec<Bot>, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    match bots
        .filter(team_id.eq(tid))
        .load::<SqlBot>(&mut conn) {
            Ok(u) => Ok(u.into_iter().map(Bot::from).collect::<Vec<Bot>>()),
            Err(e) => Err(e)
    }
}



pub fn get_bots_by_ids(ids: Vec<String>) -> Result<Vec<Bot>, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    let sql_bots = bots
        .filter(id.eq_any(ids))
        .load::<SqlBot>(&mut conn)?;
    let converted_bots: Vec<Bot> = sql_bots.into_iter()
        .map(|sql_bot| Bot::from(sql_bot))
        .collect();
    Ok(converted_bots)
}

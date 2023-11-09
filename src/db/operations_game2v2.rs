use diesel::result::Error;
use diesel::{prelude::*, insert_into};
use crate::db::schema::games_2v2::dsl::*;
use crate::models::game_2v2::{SqlGame2v2, Game2v2, NewGame2v2};
use super::operations_db::establish_connection;


pub fn insert_game(game: NewGame2v2) ->  Result<Game2v2, Error> {
    let new_bot = SqlGame2v2::from(game);
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    let _ = insert_into(games_2v2)
        .values(&new_bot)
        .execute(&mut conn)?;
    Ok(Game2v2::from(new_bot))
}

pub fn get_game_by_id(uid: String) -> Result<Game2v2, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    match games_2v2
        .filter(id.eq(uid))
        .first::<SqlGame2v2>(&mut conn) {
            Ok(t) => Ok(Game2v2::from(t)),
            Err(e) => Err(e)
    }
}

pub fn get_games_by_bot_id(bot_id: String) -> Result<Vec<Game2v2>, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    let games = games_2v2
        .filter(
            team1bot1_id.eq(bot_id.clone())
                .or(team1bot2_id.eq(bot_id.clone()))
                .or(team2bot1_id.eq(bot_id.clone()))
                .or(team2bot2_id.eq(bot_id.clone()))    
        )
        .distinct()
        .load::<SqlGame2v2>(&mut conn)?;
    Ok(games.into_iter().map(Game2v2::from).collect::<Vec<Game2v2>>())
}

pub fn get_rounds_for_competition(team_id: String, com_id: String) -> Result<Vec<Game2v2>, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    let games = games_2v2
        .filter(
            team1_id
                .eq(team_id.clone())
                .or(team2_id.eq(team_id.clone()))
        )
        .distinct()
        .filter(competition_id.eq(com_id))
        .load::<SqlGame2v2>(&mut conn)?;
    Ok(games.into_iter().map(Game2v2::from).collect::<Vec<Game2v2>>())
}

pub fn game_set_public(game_id: String, public_state: bool) -> Result<(), Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    diesel::update(games_2v2.filter(id.eq(game_id)))
        .set(public.eq(public_state))
        .execute(&mut conn)?;
    Ok(())
}

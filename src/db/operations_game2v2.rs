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

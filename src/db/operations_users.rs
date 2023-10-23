use diesel::result::Error;
use diesel::{prelude::*, insert_into};
use crate::db::schema::users::dsl::*;
use crate::models::user::{SqlUser, User, NewUser};
use super::operations_db::establish_connection;


pub fn insert_user(student: NewUser) ->  Result<User, Error> {
    let new_user = SqlUser::from(student);
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    let _ = insert_into(users)
        .values(&new_user)
        .execute(&mut conn)?;
    Ok(User::from(new_user))
}

pub fn get_user_by_id(uid: String) -> Result<User, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    match users
        .filter(id.eq(uid))
        .first::<SqlUser>(&mut conn) {
            Ok(u) => Ok(User::from(u)),
            Err(e) => Err(e)
    }
}

pub fn get_user_by_username(un: String) -> Result<User, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    match users
        .filter(username.eq(un))
        .first::<SqlUser>(&mut conn) {
            Ok(u) => Ok(User::from(u)),
            Err(e) => Err(e)
    }
}

pub fn get_user_by_studnet_number(num: String) -> Result<User, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    match users
        .filter(username.eq(num))
        .first::<SqlUser>(&mut conn) {
            Ok(u) => Ok(User::from(u)),
            Err(e) => Err(e)
    }
}

pub fn get_users_by_ids(ids: Vec<String>) -> Result<Vec<User>, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    let sql_users = users
        .filter(id.eq_any(ids))
        .load::<SqlUser>(&mut conn)?;
    let converted_users: Vec<User> = sql_users.into_iter()
        .map(|sql_user| User::from(sql_user))
        .collect();
    Ok(converted_users)
}
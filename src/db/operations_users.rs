use diesel::result::Error;
use diesel::{prelude::*, insert_into};
use crate::db::schema::users::dsl::*;
use crate::models::user::{SqlUser, User, NewUser};
use super::operations_db::establish_connection;


pub fn insert_student(student: NewUser) ->  Result<User, Error> {
    let new_student = SqlUser::from(student);
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    let _ = insert_into(users)
        .values(&new_student)
        .execute(&mut conn)?;
    Ok(User::from(new_student))
}

pub fn get_student_by_id(uid: String) -> Result<User, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    match users
        .filter(id.eq(uid))
        .first::<SqlUser>(&mut conn) {
            Ok(u) => Ok(User::from(u)),
            Err(e) => Err(e)
    }
}

pub fn get_student_by_studnet_number(num: String) -> Result<User, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    match users
        .filter(username.eq(num))
        .first::<SqlUser>(&mut conn) {
            Ok(u) => Ok(User::from(u)),
            Err(e) => Err(e)
    }
}

pub fn get_students_by_ids(ids: Vec<String>) -> Result<Vec<User>, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    let sql_students = users
        .filter(id.eq_any(ids))
        .load::<SqlUser>(&mut conn)?;
    let converted_students: Vec<User> = sql_students.into_iter()
        .map(|sql_student| User::from(sql_student))
        .collect();
    Ok(converted_students)
}
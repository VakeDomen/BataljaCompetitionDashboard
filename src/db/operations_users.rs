use diesel::result::Error;
use diesel::{prelude::*, insert_into};
use crate::db::schema::users::dsl::*;
use crate::models::student::{SqlStudent, Student, NewStudent};
use super::operations_db::establish_connection;


pub fn insert_student(student: NewStudent) ->  Result<Student, Error> {
    let new_student = SqlStudent::from(student);
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    let _ = insert_into(users)
        .values(&new_student)
        .execute(&mut conn)?;
    Ok(Student::from(new_student))
}

pub fn get_student_by_id(uid: String) -> Result<Student, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    match users
        .filter(id.eq(uid))
        .first::<SqlStudent>(&mut conn) {
            Ok(u) => Ok(Student::from(u)),
            Err(e) => Err(e)
    }
}

pub fn get_student_by_studnet_number(num: String) -> Result<Student, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    match users
        .filter(student_number.eq(num))
        .first::<SqlStudent>(&mut conn) {
            Ok(u) => Ok(Student::from(u)),
            Err(e) => Err(e)
    }
}

pub fn get_students_by_ids(ids: Vec<String>) -> Result<Vec<Student>, Error> {
    let mut conn = establish_connection().expect("Failed to get a DB connection from the pool");
    let sql_students = users
        .filter(id.eq_any(ids))
        .load::<SqlStudent>(&mut conn)?;
    let converted_students: Vec<Student> = sql_students.into_iter()
        .map(|sql_student| Student::from(sql_student))
        .collect();
    Ok(converted_students)
}
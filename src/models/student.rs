use diesel::prelude::{Insertable, Queryable};
use serde::Serialize;
use chrono::{NaiveDateTime, Local};
use uuid::Uuid;
use crate::db::schema::users::{self};


#[derive(Debug)]
pub struct LdapStudent {
    pub student_number: i32,
    pub ldap_dn: String,
}

#[derive(Debug)]
pub struct NewStudent {
    id: String,
    student_number: i32,
    ldap_dn: String,
    created: NaiveDateTime,
}

#[derive(Debug)]
pub struct Student {
    pub id: String,
    pub student_number: i32,
    pub ldap_dn: String,
    pub created: NaiveDateTime,
}   

#[derive(Queryable, Debug, Insertable)]
#[diesel(table_name = users)]
pub struct SqlStudent {
    pub id: String,
    pub student_number: String,
    pub ldap_dn: String,
    pub created: NaiveDateTime,
}

#[derive(Debug, Serialize, Clone)]
pub struct PublicStudent {
    pub id: String,
    pub student_number: String,
}

impl From<SqlStudent> for Student {
    fn from(sql_student: SqlStudent) -> Self {
        Self {
            id: sql_student.id,
            student_number: sql_student.student_number.parse().unwrap(),
            ldap_dn: sql_student.ldap_dn,
            created: sql_student.created,
        }
    }
}

impl From<Student> for PublicStudent {
    fn from(user: Student) -> Self {
        Self { 
            id: user.id, 
            student_number: user.student_number.to_string(), 
        }
    }
}

impl From<LdapStudent> for NewStudent {
    fn from(new_student: LdapStudent) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            student_number: new_student.student_number,
            ldap_dn: new_student.ldap_dn,
            created: Local::now().naive_utc(),
        }
    }
}

impl From<NewStudent> for SqlStudent {
    fn from(new_student: NewStudent) -> Self {
        Self {
            id: new_student.id,
            student_number: new_student.student_number.to_string(),
            ldap_dn: new_student.ldap_dn,
            created: new_student.created,
        }
    }
}
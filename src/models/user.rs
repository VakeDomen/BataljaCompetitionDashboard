use diesel::prelude::{Insertable, Queryable};
use serde::Serialize;
use chrono::{NaiveDateTime, Local};
use uuid::Uuid;
use crate::db::schema::users::{self};


#[derive(Debug, Clone, Serialize)]
pub enum Role {
    Student,
    Admin
}

#[derive(Debug)]
pub struct LdapUser {
    pub username: String,
    pub ldap_dn: String,
}

#[derive(Debug)]
pub struct NewUser {
    id: String,
    username: String,
    ldap_dn: String,
    role: Role,
    created: NaiveDateTime,
}

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub ldap_dn: String,
    pub role: Role,
    pub created: NaiveDateTime,
}   

#[derive(Queryable, Debug, Insertable)]
#[diesel(table_name = users)]
pub struct SqlUser {
    id: String,
    username: String,
    ldap_dn: String,
    role: String,
    created: NaiveDateTime,
}

#[derive(Debug, Serialize, Clone)]
pub struct PublicUser {
    id: String,
    username: String,
    role: Role,
}

impl From<SqlUser> for User {
    fn from(sql_user: SqlUser) -> Self {
        Self {
            id: sql_user.id,
            username: sql_user.username.parse().unwrap(),
            ldap_dn: sql_user.ldap_dn,
            role: match sql_user.role.as_str() {
                "ADMIN" => Role::Admin,
                _ => Role::Student,
            },
            created: sql_user.created,
        }
    }
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        Self { 
            id: user.id, 
            username: user.username.to_string(),
            role: user.role,
        }
    }
}

impl From<LdapUser> for NewUser {
    fn from(ldap_user: LdapUser) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            username: ldap_user.username,
            ldap_dn: ldap_user.ldap_dn,
            created: Local::now().naive_utc(),
            role: Role::Student,
        }
    }
}

impl From<NewUser> for SqlUser {
    fn from(new_user: NewUser) -> Self {
        Self {
            id: new_user.id,
            username: new_user.username.to_string(),
            ldap_dn: new_user.ldap_dn,
            created: new_user.created,
            role: match new_user.role {
                Role::Student => "STUDENT".to_string(),
                Role::Admin => "ADMIN".to_string(),
            },
        }
    }
}
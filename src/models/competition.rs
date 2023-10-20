use diesel::prelude::{Insertable, Queryable};
use serde::Serialize;
use chrono::{NaiveDateTime, Local, NaiveDate};
use uuid::Uuid;
use crate::db::schema::competitions::{self};

#[derive(Debug)]
pub struct NewCompetition {
    name: String,
    start: NaiveDate,
    end: NaiveDate,
    type_: String,
}

#[derive(Debug)]
pub struct Competition {
    pub id: String,
    pub name: String,
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub allowed_submissions: bool,
    pub round: i32,
    pub type_: String,
    pub created: NaiveDateTime,
}   

#[derive(Queryable, Debug, Insertable)]
#[diesel(table_name = competitions)]
pub struct SqlCompetition {
    pub id: String,
    pub name: String,
    pub start: String,
    pub end: String,
    pub allowed_submissions: String,
    pub round: String,
    pub type_: String,
    pub created: NaiveDateTime,
}

#[derive(Debug, Serialize, Clone)]
pub struct PublicCompetition {
    pub id: String,
    pub name: String,
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub allowed_submissions: bool,
    pub round: i32,
    pub type_: String,
    created: NaiveDateTime,
}

impl From<SqlCompetition> for Competition {
    fn from(sql_competition: SqlCompetition) -> Self {
        Self {
            id: sql_competition.id,
            name: sql_competition.name,
            start: sql_competition.start.parse().unwrap(),
            end: sql_competition.end.parse().unwrap(),
            allowed_submissions: sql_competition.allowed_submissions.parse().unwrap(),
            round: sql_competition.round.parse().unwrap(),
            type_: sql_competition.type_,
            created: sql_competition.created,
        }
    }
}

impl From<Competition> for PublicCompetition {
    fn from(competition: Competition) -> Self {
        Self { 
            id: competition.id,
            name: competition.name,
            start: competition.start,
            end: competition.end,
            allowed_submissions: competition.allowed_submissions,
            round: competition.round,
            type_: competition.type_,
            created: competition.created,
        }
    }
}

impl From<NewCompetition> for SqlCompetition {
    fn from(new_competition: NewCompetition) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: new_competition.name,
            start: new_competition.start.to_string(),
            end: new_competition.end.to_string(),
            allowed_submissions: true.to_string(),
            round: 0.to_string(),
            type_: new_competition.type_,
            created: Local::now().naive_utc(),
        }
    }
}
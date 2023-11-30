use diesel::prelude::{Insertable, Queryable};
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, Local};
use uuid::Uuid;
use crate::db::schema::teams::{self};

#[derive(Debug, Deserialize)]
pub enum BotSelector {
    First,
    Second
}

#[derive(Debug, Deserialize)]
pub struct NewTeam {
    pub name: String,
    pub owner: String,  
    pub competition_id: String,
}

#[derive(Debug, Clone)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub partner: String,
    pub competition_id: String,
    pub bot1: String,
    pub bot2: String,
    pub elo: i32,
    pub created: NaiveDateTime,
}   

#[derive(Queryable, Debug, Insertable)]
#[diesel(table_name = teams)]
pub struct SqlTeam {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub partner: String,
    pub competition_id: String,
    pub bot1: String,
    pub bot2: String,
    pub elo: i32,
    pub created: NaiveDateTime,
}

#[derive(Debug, Serialize, Clone)]
pub struct PublicTeam {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub partner: String,
    pub competition_id: String,
    pub bot1: String,
    pub bot2: String,
    pub elo: i32,
    pub created: NaiveDateTime,
}

impl From<SqlTeam> for Team {
    fn from(sql_team: SqlTeam) -> Self {
        Self {
            id: sql_team.id,
            name: sql_team.name,
            owner: sql_team.owner,
            partner: sql_team.partner,
            competition_id: sql_team.competition_id,
            bot1: sql_team.bot1,
            bot2: sql_team.bot2,
            elo: sql_team.elo,
            created: sql_team.created,
        }
    }
}

impl From<Team> for PublicTeam {
    fn from(team: Team) -> Self {
        Self { 
            id: team.id,
            name: team.name,
            owner: team.owner,
            partner: team.partner,
            competition_id: team.competition_id,
            bot1: team.bot1,
            bot2: team.bot2,
            elo: team.elo,
            created: team.created,
        }
    }
}

impl From<NewTeam> for SqlTeam {
    fn from(new_team: NewTeam) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: new_team.name,
            owner: new_team.owner,
            partner: "".to_string(),
            competition_id: new_team.competition_id,
            bot1: "".to_string(),
            bot2: "".to_string(),
            elo: 1000,
            created: Local::now().naive_utc(),
        }
    }
}
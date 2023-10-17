use diesel::prelude::{Insertable, Queryable};
use serde::Serialize;
use chrono::{NaiveDateTime, Local};
use uuid::Uuid;
use crate::db::schema::teams::{self};

#[derive(Debug)]
pub struct NewTeam {
    owner: String,
    partner: String,
    competition_id: String,
    bot1: String,
    bot2: String,
}

#[derive(Debug)]
pub struct Team {
    pub id: String,
    pub owner: String,
    pub partner: String,
    pub competition_id: String,
    pub bot1: String,
    pub bot2: String,
    pub created: NaiveDateTime,
}   

#[derive(Queryable, Debug, Insertable)]
#[diesel(table_name = teams)]
pub struct SqlTeam {
    pub id: String,
    pub owner: String,
    pub partner: String,
    pub competition_id: String,
    pub bot1: String,
    pub bot2: String,
    pub created: NaiveDateTime,
}

#[derive(Debug, Serialize, Clone)]
pub struct PublicTeam {
    pub id: String,
    pub owner: String,
    pub partner: String,
    pub competition_id: String,
    pub bot1: String,
    pub bot2: String,
    pub created: NaiveDateTime,
}

impl From<SqlTeam> for Team {
    fn from(sql_team: SqlTeam) -> Self {
        Self {
            id: sql_team.id,
            owner: sql_team.owner,
            partner: sql_team.partner,
            competition_id: sql_team.competition_id,
            bot1: sql_team.bot1,
            bot2: sql_team.bot2,
            created: sql_team.created,
        }
    }
}

impl From<Team> for PublicTeam {
    fn from(team: Team) -> Self {
        Self { 
            id: team.id,
            owner: team.owner,
            partner: team.partner,
            competition_id: team.competition_id,
            bot1: team.bot1,
            bot2: team.bot2,
            created: team.created,
        }
    }
}

impl From<NewTeam> for SqlTeam {
    fn from(new_team: NewTeam) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            owner: new_team.owner,
            partner: new_team.partner,
            competition_id: new_team.competition_id,
            bot1: new_team.bot1,
            bot2: new_team.bot2,
            created: Local::now().naive_utc(),
        }
    }
}
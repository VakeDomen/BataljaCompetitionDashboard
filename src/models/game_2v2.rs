use diesel::prelude::{Insertable, Queryable};
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, Local};
use uuid::Uuid;
use crate::db::schema::games_2v2::{self};

#[derive(Debug, Deserialize)]
pub struct NewGame2v2 {
    pub id: String,
    pub competition_id: String,
    pub round: i32,
    pub team1_id: String,
    pub team2_id: String,
    pub winner_id: String,
    pub team1bot1_id: String,
    pub team1bot2_id: String,
    pub team2bot1_id: String,
    pub team2bot2_id: String,
    pub team1bot1_survived: bool,
    pub team1bot2_survived: bool,
    pub team2bot1_survived: bool,
    pub team2bot2_survived: bool,
    pub log_file_path: String,
    pub public: bool,
    pub additional_data: String,
}

#[derive(Debug)]
pub struct Game2v2 {
    pub id: String,
    pub competition_id: String,
    pub round: i32,
    pub team1_id: String,
    pub team2_id: String,
    pub winner_id: String,
    pub team1bot1_id: String,
    pub team1bot2_id: String,
    pub team2bot1_id: String,
    pub team2bot2_id: String,
    pub team1bot1_survived: bool,
    pub team1bot2_survived: bool,
    pub team2bot1_survived: bool,
    pub team2bot2_survived: bool,
    pub log_file_path: String,
    pub public: bool,
    pub additional_data: String,
    pub created: NaiveDateTime,
}   

#[derive(Queryable, Debug, Insertable)]
#[diesel(table_name = games_2v2)]
pub struct SqlGame2v2 {
    pub id: String,
    pub competition_id: String,
    pub round: i32,
    pub team1_id: String,
    pub team2_id: String,
    pub winner_id: String,
    pub team1bot1_id: String,
    pub team1bot2_id: String,
    pub team2bot1_id: String,
    pub team2bot2_id: String,
    pub team1bot1_survived: bool,
    pub team1bot2_survived: bool,
    pub team2bot1_survived: bool,
    pub team2bot2_survived: bool,
    pub log_file_path: String,
    pub public: bool,
    pub additional_data: String,
    pub created: NaiveDateTime,
}

#[derive(Debug, Serialize, Clone)]
pub struct PublicGame2v2 {
    pub id: String,
    pub competition_id: String,
    pub round: i32,
    pub team1_id: String,
    pub team2_id: String,
    pub winner_id: String,
    pub team1bot1_id: String,
    pub team1bot2_id: String,
    pub team2bot1_id: String,
    pub team2bot2_id: String,
    pub team1bot1_survived: bool,
    pub team1bot2_survived: bool,
    pub team2bot1_survived: bool,
    pub team2bot2_survived: bool,
    pub public: bool,
    pub additional_data: String,
    pub created: NaiveDateTime,
}

impl From<SqlGame2v2> for Game2v2 {
    fn from(sql_game_2v2: SqlGame2v2) -> Self {
        Self {
            id: sql_game_2v2.id,
            competition_id: sql_game_2v2.competition_id,
            round: sql_game_2v2.round,
            team1_id: sql_game_2v2.team1_id,
            team2_id: sql_game_2v2.team2_id,
            winner_id: sql_game_2v2.winner_id,
            team1bot1_id: sql_game_2v2.team1bot1_id,
            team1bot2_id: sql_game_2v2.team1bot2_id,
            team2bot1_id: sql_game_2v2.team2bot1_id,
            team2bot2_id: sql_game_2v2.team2bot2_id,
            team1bot1_survived: sql_game_2v2.team1bot1_survived,
            team1bot2_survived: sql_game_2v2.team1bot2_survived,
            team2bot1_survived: sql_game_2v2.team2bot1_survived,
            team2bot2_survived: sql_game_2v2.team2bot2_survived,
            log_file_path: sql_game_2v2.log_file_path,
            public: sql_game_2v2.public,
            additional_data: sql_game_2v2.additional_data,
            created: sql_game_2v2.created,
        }
    }
}

impl From<Game2v2> for PublicGame2v2 {
    fn from(game_2v2: Game2v2) -> Self {
        Self { 
            id: game_2v2.id,
            competition_id: game_2v2.competition_id,
            round: game_2v2.round,
            team1_id: game_2v2.team1_id,
            team2_id: game_2v2.team2_id,
            winner_id: game_2v2.winner_id,
            team1bot1_id: game_2v2.team1bot1_id,
            team1bot2_id: game_2v2.team1bot2_id,
            team2bot1_id: game_2v2.team2bot1_id,
            team2bot2_id: game_2v2.team2bot2_id,
            team1bot1_survived: game_2v2.team1bot1_survived,
            team1bot2_survived: game_2v2.team1bot2_survived,
            team2bot1_survived: game_2v2.team2bot1_survived,
            team2bot2_survived: game_2v2.team2bot2_survived,
            public: game_2v2.public,
            additional_data: game_2v2.additional_data,
            created: game_2v2.created,
        }
    }
}

impl From<NewGame2v2> for SqlGame2v2 {
    fn from(new_game_2v2: NewGame2v2) -> Self {
        Self {
            id: new_game_2v2.id,
            competition_id: new_game_2v2.competition_id,
            round: new_game_2v2.round,
            team1_id: new_game_2v2.team1_id,
            team2_id: new_game_2v2.team2_id,
            winner_id: new_game_2v2.winner_id,
            team1bot1_id: new_game_2v2.team1bot1_id,
            team1bot2_id: new_game_2v2.team1bot2_id,
            team2bot1_id: new_game_2v2.team2bot1_id,
            team2bot2_id: new_game_2v2.team2bot2_id,
            team1bot1_survived: new_game_2v2.team1bot1_survived,
            team1bot2_survived: new_game_2v2.team1bot2_survived,
            team2bot1_survived: new_game_2v2.team2bot1_survived,
            team2bot2_survived: new_game_2v2.team2bot2_survived,
            log_file_path: new_game_2v2.log_file_path,
            public: new_game_2v2.public,
            additional_data: new_game_2v2.additional_data,
            created: Local::now().naive_utc(),
        }
    }
}

impl NewGame2v2 {
    pub fn new(
        competition_id: String, 
        round: i32,
        team1_id: String,
        team2_id: String,
        team1bot1_id: String,
        team1bot2_id: String,
        team2bot1_id: String,
        team2bot2_id: String,
    ) -> Self {
        let id = Uuid::new_v4().to_string();
        let log_file_path = format!("./resources/games/{}/{}.txt", round, id.clone());
        Self {
            id,
            competition_id,
            round,
            team1_id,
            team2_id,
            winner_id: "".to_string(),
            team1bot1_id,
            team1bot2_id,
            team2bot1_id,
            team2bot2_id,
            team1bot1_survived: true,
            team1bot2_survived: true,
            team2bot1_survived: true,
            team2bot2_survived: true,
            log_file_path,
            public: false,
            additional_data: "".to_string(),
        }
    }
}
use std::path::Path;

use diesel::prelude::{Insertable, Queryable};
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, Local};
use uuid::Uuid;
use crate::db::schema::bots::{self};

#[derive(Debug, Deserialize)]
pub struct NewBot {
    pub team_id: String,
    pub source_path: String,
}

#[derive(Debug, Clone)]
pub struct Bot {
    pub id: String,
    pub team_id: String,
    pub bot_name: String,
    pub source_path: String,
    pub compile_error: String,
    pub created: NaiveDateTime,
}   

#[derive(Queryable, Debug, Insertable)]
#[diesel(table_name = bots)]
pub struct SqlBot {
    pub id: String,
    pub team_id: String,
    pub bot_name: String,
    pub source_path: String,
    pub compile_error: String,
    pub created: NaiveDateTime,
}

#[derive(Debug, Serialize, Clone)]
pub struct PublicBot {
    pub id: String,
    pub team_id: String,
    pub bot_name: String,
    pub compile_error: String,
    pub created: NaiveDateTime,
}

impl From<SqlBot> for Bot {
    fn from(sql_bot: SqlBot) -> Self {
        Self {
            id: sql_bot.id,
            team_id: sql_bot.team_id,
            bot_name: sql_bot.bot_name,
            source_path: sql_bot.source_path,
            compile_error: sql_bot.compile_error,
            created: sql_bot.created,
        }
    }
}

impl From<Bot> for PublicBot {
    fn from(bot: Bot) -> Self {
        Self { 
            id: bot.id,
            team_id: bot.team_id,
            bot_name: bot.bot_name,
            compile_error: bot.compile_error,
            created: bot.created,
        }
    }
}

impl From<NewBot> for SqlBot {
    fn from(new_bot: NewBot) -> Self {
        let bot_name = Path::new(&new_bot.source_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();
        
        Self {
            id: Uuid::new_v4().to_string(),
            team_id: new_bot.team_id,
            bot_name,
            source_path: new_bot.source_path,
            compile_error: "".to_string(),
            created: Local::now().naive_utc(),
        }
    }
}
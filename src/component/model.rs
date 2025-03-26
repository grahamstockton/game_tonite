use chrono::Utc;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::dao::sqlite_util::UserRecord;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub picture: String, // TODO: put picture here,
}

impl User {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_picture(&self) -> String {
        self.picture.clone()
    }
}

#[cfg(feature = "ssr")]
impl From<&UserRecord> for User {
    fn from(record: &UserRecord) -> Self {
        Self {
            name: record.user_id.clone(),
            picture: record.user_photo.clone(),
        }
    }
}

#[derive(Clone, Serialize, Debug, Store, Deserialize)]
pub struct GamingSession {
    pub server_id: String,
    pub session_id: i64,
    pub title: String,
    pub start_time: chrono::DateTime<Utc>,
    pub end_time: chrono::DateTime<Utc>,
    pub owner: User,
    pub participants: Vec<User>,
    //selected_games: Vec<Game>, // todo: fill game in here
    //suggested_games: Vec<Game>,
}

pub struct Game {
    pub title: String,
    pub cover_url: String,
}

impl Game {
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_cover_url(&self) -> String {
        self.cover_url.clone()
    }
}

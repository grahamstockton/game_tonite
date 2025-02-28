use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct GamingSession {
    pub server_id: i64,
    pub session_id: i64,
    pub start_time: chrono::DateTime<Utc>,
    pub end_time: chrono:: DateTime<Utc>,
    pub owner: User,
    pub other_participants: Vec<User>,
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
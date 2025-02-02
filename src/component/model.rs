use chrono::{DateTime, Utc};

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

pub struct GamingSession {
    server_id: String,
    session_id: String,
    start_time: chrono::DateTime<Utc>,
    end_time: chrono:: DateTime<Utc>,
    owner: User,
    other_participants: Vec<User>,
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
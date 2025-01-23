use tokio::time::Instant;

pub struct User {
    name: String,
    picture: String, // TODO: put picture here,
}

pub struct GamingSession {
    server_id: String,
    session_id: String,
    start_time: Instant,
    owner: User,
    other_participants: Vec<User>,
    selected_games: Vec<Game>, // todo: fill game in here
    suggested_games: Vec<Game>,
}
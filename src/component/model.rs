use std:;time::Instant;

pub struct CalendarEvent {
    start_time: Instant,
    end_time: Instant,
}

pub struct User {
    name: String,
    picture: String, // TODO: put picture here,
}

pub struct GamingSession {
    calendar_event: CalendarEvent,
    owner: User,
    other_participants: Vec<User>,
    selected_game: Option<Game>, // todo: fill game in here
    suggested_games: Vec<Game>,

}
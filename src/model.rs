use serde;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Game {
    pub title: String,
    pub cover_id: usize,
    pub cover_url: Option<String>,
}

#[cfg(feature = "ssr")]
use chrono::Utc;
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use anyhow::Result;
        use sqlx::prelude::FromRow;
        use sqlx::{Pool, Sqlite, SqlitePool};
        use chrono::DateTime;
    }
}

#[cfg(feature = "ssr")]
#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct SessionRecord {
    pub session_id: Option<i64>,
    pub server_id: String,
    pub title: String,
    pub start_time: String,
    pub end_time: String,
    pub owner: String,
    pub is_selected: bool,
}

#[cfg(feature = "ssr")]
#[derive(Clone, FromRow, Debug)]
pub struct UserRecord {
    pub session_id: i64,
    pub user_id: String,
    pub user_photo: String,
}

#[cfg(feature = "ssr")]
#[derive(Clone, FromRow, Debug)]
pub struct GamePreferenceRecord {
    pub id: i64,
    pub user_id: String,
    pub session_id: i64,
    pub suggested_game: String,
    pub is_selected: bool,
}

#[cfg(feature = "ssr")]
pub struct SqliteClient {
    client: Pool<Sqlite>,
}

#[cfg(feature = "ssr")]
impl SqliteClient {
    pub async fn new(db_url: &str) -> Self {
        Self {
            client: SqlitePool::connect(db_url).await.unwrap(),
        }
    }

    // session table -- CREATE
    pub async fn create_session(
        &self,
        server_id: &str,
        title: &str,
        start_time: &str,
        end_time: &str,
        owner: &str,
        is_selected: bool,
    ) -> Result<SessionRecord> {
        let record = sqlx::query_as!(SessionRecord,
            "INSERT INTO sessions (server_id, title, start_time, end_time, owner, is_selected) VALUES (?, ?, ?, ?, ?, ?) RETURNING *",
            server_id,
            title,
            start_time,
            end_time,
            owner,
            is_selected
        ).fetch_optional(&self.client).await;

        match record {
            Ok(o) => Ok(o.unwrap()),
            Err(e) => Err(e.into()),
        }
    }

    // session table -- READ multiple
    pub async fn get_sessions(&self, server_id: &str) -> Result<Vec<SessionRecord>> {
        Ok(sqlx::query_as!(
            SessionRecord,
            "SELECT * FROM sessions WHERE server_id=?",
            server_id
        )
        .fetch_all(&self.client)
        .await?)
    }

    // session table -- read, only for one day
    pub async fn get_sessions_in_range(
        &self,
        server_id: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<SessionRecord>> {
        let start = start_time.to_rfc3339();
        let end = end_time.to_rfc3339();
        Ok(sqlx::query_as!(
            SessionRecord,
            "SELECT * FROM sessions WHERE server_id=? AND start_time BETWEEN ? AND ?",
            server_id,
            start,
            end
        )
        .fetch_all(&self.client)
        .await?)
    }

    // session table -- READ one
    pub async fn get_session(&self, session_id: i64) -> Result<Option<SessionRecord>> {
        Ok(sqlx::query_as!(
            SessionRecord,
            "SELECT * FROM sessions WHERE session_id=?",
            session_id
        )
        .fetch_optional(&self.client)
        .await?)
    }

    // session table -- DELETE
    pub async fn delete_session(&self, session_id: i64) -> Result<()> {
        let _ = sqlx::query!("DELETE FROM sessions WHERE session_id=?", session_id)
            .execute(&self.client)
            .await?;

        Ok(())
    }

    // user table -- CREATE
    pub async fn create_session_user(
        &self,
        user_id: &str,
        session_id: i64,
        user_photo: &str,
    ) -> Result<()> {
        let _ = sqlx::query!(
            "INSERT INTO users (user_id, session_id, user_photo) VALUES (?, ?, ?)",
            user_id,
            session_id,
            user_photo
        )
        .execute(&self.client)
        .await?;

        Ok(())
    }

    // user table -- READ
    pub async fn get_session_users(&self, session_id: i64) -> Result<Vec<UserRecord>> {
        Ok(sqlx::query_as!(
            UserRecord,
            "SELECT * FROM users WHERE session_id=?",
            session_id
        )
        .fetch_all(&self.client)
        .await?)
    }

    // user table -- DELETE
    pub async fn delete_session_user(&self, session_id: i64, user_id: &str) -> Result<()> {
        let _ = sqlx::query!(
            "DELETE FROM users WHERE session_id=? AND user_id=?",
            session_id,
            user_id
        )
        .execute(&self.client)
        .await?;

        Ok(())
    }

    // preference table -- CREATE
    pub async fn create_game_preference_record(
        &self,
        user_id: &str,
        session_id: i64,
        suggested_game: &str,
        is_selected: bool,
    ) -> Result<()> {
        let _ = sqlx::query!("INSERT INTO preferences (user_id, session_id, suggested_game, is_selected) VALUES (?, ?, ?, ?)",
            user_id,
            session_id,
            suggested_game,
            is_selected,
        ).execute(&self.client).await?;

        Ok(())
    }

    // preference table -- READ
    pub async fn get_game_preference_records(
        &self,
        user_id: &str,
        session_id: i64,
    ) -> Result<Vec<GamePreferenceRecord>> {
        Ok(sqlx::query_as!(
            GamePreferenceRecord,
            "SELECT * FROM preferences WHERE user_id=? AND session_id=?",
            user_id,
            session_id
        )
        .fetch_all(&self.client)
        .await?)
    }

    // preference table -- DELETE
    pub async fn delete_game_preference_record(
        &self,
        user_id: &str,
        session_id: i64,
    ) -> Result<()> {
        let _ = sqlx::query!(
            "DELETE FROM preferences WHERE user_id=? AND session_id=?",
            user_id,
            session_id
        )
        .execute(&self.client)
        .await?;

        Ok(())
    }
}

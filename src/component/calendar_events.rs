use std::sync::Arc;

use chrono::{DateTime, Utc};
use futures::future::join_all;
use leptos::{logging::log, prelude::*};

use crate::component::{
    event_card::EventCard,
    model::{Game, User},
};

use super::model::GamingSession;

/**
 * Gets calendar events from sqlite. Creates event cards for each
 */
#[component]
pub fn CalendarEvents() -> impl IntoView {
    view! {
        <div class="absolute top-0">
            <Await
                future=get_events("PLACEHOLDER".to_string())
                let:res
            >
                {
                    let empty_vec: Vec<GamingSession> = vec![];
                    res.as_ref().unwrap_or_else(|_| &empty_vec).iter().map(|r| view! {
                        <div class="z-1 absolute top-100">
                            <EventCard
                                title={r.title.clone()} // not this one
                                selected_game={Some(Arc::new(Game { title: "placeholder".to_string(), cover_url: "url".to_string()}))} // not this one
                                owner={Arc::new(r.owner.clone())}
                                participants={r.participants.iter().map(|i| Arc::new(i.clone())).collect()}
                                suggestions={vec![]}
                            />
                        </div>
                    }).collect_view()
                }
            </Await>
        </div>
    }
}

#[server]
async fn get_events(server_id: String) -> Result<Vec<GamingSession>, ServerFnError> {
    use crate::dao::sqlite_util::SqliteClient;
    // TODO: test this, then use extractors to share an sqlite client across instances
    let client = SqliteClient::new("sqlite://sessions.db").await;
    let sessions = client.get_sessions(&server_id).await.unwrap();
    log!("getting events: {}", Utc::now());

    // TODO: make this call process faster
    let a: Vec<_> = sessions
        .iter()
        .map(|s| async {
            let participants = client.get_session_users(s.session_id).await.unwrap();
            let owner_record = participants
                .iter()
                .find(|r| s.owner == r.user_id)
                .expect("no owner found for session");

            GamingSession {
                server_id: s.server_id.clone(),
                session_id: s.session_id,
                title: s.title.clone(),
                start_time: DateTime::parse_from_rfc3339(&s.start_time)
                    .unwrap()
                    .to_utc(),
                end_time: DateTime::parse_from_rfc3339(&s.end_time).unwrap().to_utc(),
                owner: User::from(owner_record),
                participants: participants.iter().map(User::from).collect(),
            }
        })
        .collect();

    Ok(join_all(a).await)
}

use std::sync::Arc;

use chrono::{DateTime, Duration, FixedOffset, Utc};
use futures::future::join_all;
use leptos::{logging::log, prelude::*};
use reactive_stores::Store;

use crate::{
    app::{GlobalState, GlobalStateStoreFields},
    component::{
        event_card::EventCard,
        model::{Game, User},
        time_util::get_events_stacking,
    },
};

use super::model::GamingSession;

/**
 * Gets calendar events from sqlite. Creates event cards for each
 */
#[component]
pub fn CalendarEvents(
    baseline: ReadSignal<Option<DateTime<FixedOffset>>>,
    offset: usize,
) -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let url_params = state.url_params().get_untracked().get_server_id();
    let calendar_events = state.calendar_events();
    let events = move || calendar_events.get();

    view! {
        <Show
            when=move || { baseline().is_some() }
            fallback=|| view! {}
        >
        {
            let baseline_date = move || baseline().unwrap();
            let window_start = baseline_date() + Duration::hours(offset as i64);
            let window_end = baseline_date() + Duration::hours(24 + offset as i64);
            view! {
                <Await
                    future=get_events(url_params.clone(), window_start, window_end)
                    let:res
                >
                    {
                        // if successful, update events signal
                        if let Ok(v) = res.as_ref() {
                            calendar_events.set(v.clone());
                        }
                        let events_stacking = get_events_stacking(&events());
                        view! {
                            <For
                                each=move || calendar_events.get()
                                key=|s| s.server_id.clone()
                                let:r
                            >
                                <EventCard
                                    title={r.title.clone()}
                                    selected_game={Some(Arc::new(Game { title: "placeholder".to_string(), cover_url: "url".to_string()}))}
                                    owner={Arc::new(r.owner.clone())}
                                    participants={r.participants.iter().map(|i| Arc::new(i.clone())).collect()}
                                    suggestions={vec![]}
                                    start_time={r.start_time.fixed_offset()}
                                    end_time={r.end_time.fixed_offset()}
                                    baseline={ baseline_date() }
                                    stacking_col={ events_stacking.get(&r.session_id).unwrap().clone() }
                                    offset={offset}
                                />
                            </For>
                        }
                    }
                </Await>
            }
        }
        </Show>
    }.into_any()
}

#[server]
async fn get_events(
    server_id: String,
    start_time: DateTime<FixedOffset>,
    end_time: DateTime<FixedOffset>,
) -> Result<Vec<GamingSession>, ServerFnError> {
    use crate::dao::sqlite_util::SqliteClient;
    // TODO: test this, then use extractors to share an sqlite client across instances
    let client = SqliteClient::new("sqlite://sessions.db").await;
    let sessions = client
        .get_sessions_in_range(&server_id, start_time.to_utc(), end_time.to_utc())
        .await
        .unwrap();
    log!("getting events: {}", Utc::now());

    // TODO: make this call process faster
    let a: Vec<_> = sessions
        .iter()
        .map(|s| async {
            let session_id = s.session_id.unwrap();
            let participants = client.get_session_users(session_id).await.unwrap();
            /*let owner_record = participants
            .iter()
            .find(|r| s.owner == r.user_id)
            .unwrap_or(&UserRecord {
                session_id: 1,
                user_id: "bobbo".to_string(),
                user_photo: "photo".to_string(),
            });*/

            GamingSession {
                server_id: s.server_id.clone(),
                session_id: session_id,
                title: s.title.clone(),
                start_time: DateTime::parse_from_rfc3339(&s.start_time)
                    .unwrap()
                    .to_utc(),
                end_time: DateTime::parse_from_rfc3339(&s.end_time).unwrap().to_utc(),
                owner: User {
                    name: "bobert".to_string(),
                    picture: "alabama".to_string(),
                },
                participants: participants.iter().map(User::from).collect(),
            }
        })
        .collect();

    Ok(join_all(a).await)
}

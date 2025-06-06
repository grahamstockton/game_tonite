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
    obf_util::UrlParamsStoreFields,
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
    // unpack state
    let state = expect_context::<Store<GlobalState>>();
    let server_id = state.url_params().server_id().get_untracked();
    let user_id = move || state.url_params().user_id().get_untracked();
    let calendar_events = state.calendar_events();

    // create stacking for display
    let events_stacking = move || get_events_stacking(&calendar_events.get());

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
                    future=get_events(server_id.clone(), window_start, window_end)
                    let:res
                >
                    {
                        // if successful, update events signal
                        if let Ok(v) = res.as_ref() {
                            calendar_events.set(v.clone());
                        }
                        move || calendar_events.get().iter().map(|r| view! {
                            <EventCard
                                title={r.title.clone()}
                                owner={Arc::new(r.owner.clone())}
                                participants={r.participants.iter().map(|i| Arc::new(i.clone())).collect()}
                                start_time={r.start_time.fixed_offset()}
                                end_time={r.end_time.fixed_offset()}
                                baseline={ baseline_date() }
                                stacking_col={ events_stacking().get(&r.session_id).unwrap().clone() }
                                session_id={r.session_id.clone()}
                                game={r.game.clone()}
                                user_id={user_id()}
                                offset={offset}
                            />
                        }).collect_view()
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
    use sqlx::{Pool, Sqlite};

    let pool = use_context::<Pool<Sqlite>>().expect("pool not found");
    let client = SqliteClient::from_pool(pool).await;

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
            let owner_record = participants
                .iter()
                .find(|r| s.owner == r.user_id)
                .expect("no owner found for session");

            GamingSession {
                server_id: s.server_id.clone(),
                session_id: s.session_id.unwrap(),
                title: s.title.clone(),
                start_time: DateTime::parse_from_rfc3339(&s.start_time)
                    .unwrap()
                    .to_utc(),
                end_time: DateTime::parse_from_rfc3339(&s.end_time).unwrap().to_utc(),
                owner: User::from(owner_record),
                participants: participants.iter().map(User::from).collect(),
                game: s.game.clone(),
            }
        })
        .collect();

    Ok(join_all(a).await)
}

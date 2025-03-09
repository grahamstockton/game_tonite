use std::sync::Arc;

use chrono::{DateTime, FixedOffset, TimeZone, Timelike, Utc};
use futures::{future::join_all, io::empty};
use leptos::{html::Div, logging::log, prelude::*};
use leptos_use::{
    use_element_size, use_interval_fn, use_scroll, use_window_size, UseElementSizeReturn,
    UseScrollReturn, UseWindowSizeReturn,
};

use crate::component::{event_card::EventCard, model::Game};

use super::model::{GamingSession, User};

#[component]
pub fn Calendar() -> impl IntoView {
    // some constants to do with displaying calendar
    const STARTING_HOUR_OFFSET: usize = 6;
    const HORIZONTAL_LINE_OFFSET: &str = "1.25rem";
    const SCROLL_OFFSET_PCT: f64 = 0.25;

    // get client side time
    let (time, set_time) =
        signal::<DateTime<FixedOffset>>(DateTime::from_timestamp(0, 0).unwrap().fixed_offset());
    let (timebar_bottom, set_timebar_bottom) = signal(0.);

    // node ref for scrolling
    let e = NodeRef::<Div>::new();
    let e2 = NodeRef::<Div>::new();
    let UseScrollReturn { set_y, .. } = use_scroll(e);
    let UseElementSizeReturn { height, .. } = use_element_size(e2);
    let UseWindowSizeReturn {
        height: window_height,
        ..
    } = use_window_size();

    // On render (client side) update time via effect.
    // Unfortunately, `use_interval_fn_with_options` initializes before the component renders
    // so this is necessary.
    Effect::watch(
        move || height(),
        move |h, _, _| {
            // set time locally
            let t = get_local_time();
            set_time(t);

            // set current time bar locally
            let tb = calculate_timebar_bottom(t, STARTING_HOUR_OFFSET);
            set_timebar_bottom(tb);

            // set screen scroll position
            let sy =
                move || (100. - tb) / 100. * h - SCROLL_OFFSET_PCT * window_height.get_untracked();
            set_y(sy());
        },
        false,
    );

    // Update time every 30s
    let _ = use_interval_fn(
        move || {
            let t = get_local_time();
            set_time(t);
            set_timebar_bottom(calculate_timebar_bottom(t, STARTING_HOUR_OFFSET));
        },
        30000,
    );

    view! {
        <div node_ref=e class="relative flex flex-col h-dvh w-dvw overflow-y-scroll">
            <div node_ref=e2 class="relative flex-shrink-0">
                // background -- hour grid
                {
                    (0..24).map(|h| {
                        let v = (h + STARTING_HOUR_OFFSET) % 24;
                        view! {
                            <div class="h-24 flex-shrink-0">
                                <hr class="z-0 border-contrast"/>
                                <p class="z-0 pl-2 text-contrast">{format!("{:0>2}:00", v)}</p>
                            </div>
                        }
                    }).collect_view()
                }
                // foreground layer -- event components
                { //TODO: make this not run 3 times for every page reload
                    move || {
                        view! {
                            <Await
                                future=get_events("PLACEHOLDER".to_string())
                                let:res
                            >
                                {
                                    let empty_vec: Vec<GamingSession> = vec![];
                                    res.as_ref().clone().unwrap_or_else(|_| &empty_vec).iter().map(|r| view! {
                                        <div class="z-1 absolute">
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
                        }
                    }
                }

                // overlay -- current time indicator
                <div class="absolute w-full flex-shrink-0" style={move || format!("bottom: {}%;", timebar_bottom()) }>
                    <p class="text-sm pr-2 text-right z-2 text-accent">{ move || format!("{}", time().format("%H:%M")) }</p>
                    <div class="z-2 divider divider-accent h-px m-0"></div>
                </div>
            </div>
        </div>
    }
}

// Given a date and a number of hours offset for the display, return bottom padding
fn calculate_timebar_bottom(t: DateTime<FixedOffset>, offset: usize) -> f64 {
    let nsfm = t.num_seconds_from_midnight() as f64;
    let offset_secs = offset as f64 * 3600.;

    if nsfm < offset_secs {
        100. * (offset_secs - nsfm) / 86400.
    } else {
        100. * (1. - (nsfm - offset_secs) / 86400.)
    }
}

// get the local time with timezone from the client
fn get_local_time() -> DateTime<FixedOffset> {
    let mins_offset = js_sys::Date::new_0().get_timezone_offset();
    let offset = FixedOffset::west_opt((mins_offset * 60.) as i32).unwrap();

    offset.from_utc_datetime(&Utc::now().naive_utc())
}

#[server]
async fn get_events(server_id: String) -> Result<Vec<GamingSession>, ServerFnError> {
    use crate::dao::sqlite_util::SqliteClient;
    // TODO: test this, then use extractors to share an sqlite client across instances
    let client = SqliteClient::new("sqlite://sessions.db").await;
    let sessions = client.get_sessions(&server_id).await.unwrap();
    log!("getting events");

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

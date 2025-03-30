use std::sync::Arc;

use chrono::{DateTime, FixedOffset};
use leptos::{logging::log, prelude::*};

use crate::component::{
    join_leave_session_button::JoinLeaveSessionButton, modal::delete_event_modal::DeleteEventModal,
    time_util::calculate_time_pct,
};

use super::model::{Game, User};

/**
 * Display component for an event.
 */
#[component]
pub fn EventCard(
    title: String,
    owner: Arc<User>,
    participants: Vec<Arc<User>>,
    start_time: DateTime<FixedOffset>,
    end_time: DateTime<FixedOffset>,
    baseline: DateTime<FixedOffset>,
    stacking_col: i32,
    session_id: i64,
    user_id: String,
    game: Option<String>,
    offset: usize,
) -> impl IntoView {
    let is_user_owner = user_id == owner.get_name();

    let game_selected = game.is_some();
    let start_pct = calculate_time_pct(start_time, baseline, offset);
    let end_pct = calculate_time_pct(end_time, baseline, offset);
    log!("creating event card");

    view! {
        <div style={ format!("position: absolute; display: flex; top: {}%; bottom: {}%; left: {}rem;", start_pct * 100., (1. - end_pct) * 100., 4 + stacking_col * 12) }>
            <div class="relative z-1 w-48 h-full card bg-primary card-border border-primary-content shadow-sm">
                <div class="card-body">
                    <div class="absolute top-2 right-2">
                        <DeleteEventModal session_id={session_id} owner_id={owner.get_name()}/>
                    </div>
                    <h2 class="text-xl font-bold card-title">{ title }</h2>
                    // game title if game selected
                    {
                        if game_selected {
                            view! {
                                <div class="flex flex-row gap-1">
                                    <h2>Game: </h2>
                                    <span>{game.unwrap()}</span>
                                </div>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }
                    }
                    // owner
                    <div class="flex flex-row gap-1">
                        <h2>Owner: </h2>
                        <span class="text-sm">{ owner.get_name() }</span>
                    </div>
                    // participants
                    <div class="avatar-group bg-primary -space-x-4">
                        {
                            participants.iter()
                            .map(|p| view! {
                                <div class="avatar border-primary border-2">
                                    <div class="w-8">
                                        <img
                                            src={ get_url() }
                                            alt={format!("{}'s profile picture", p.get_name())}
                                            loading="eager"
                                        />
                                    </div>
                                </div>
                            })
                            .collect_view()
                        }
                        <div class="flex flex-row">
                            <div class="flex-1">
                            {
                                if participants.len() > 4 {
                                    view! {
                                        <div class="avatar avatar-placeholder border-primary border-2">
                                            <div class="bg-neutral text-neutral-content w-8">
                                                <span>{ format!("+{}", participants.len() - 1) }</span>
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {}.into_any()
                                }
                            }
                            </div>
                            {
                                // todo: decouple this (user id from username)
                                if !is_user_owner {
                                    view! {
                                        <JoinLeaveSessionButton session_id={session_id} />
                                    }.into_any()
                                } else {
                                    view! {}.into_any()
                                }
                            }
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }.into_any()
}

fn get_url() -> String {
    "https://wallpapers.com/images/featured/discord-profile-pictures-xk3qyllfj1j46kte.jpg"
        .to_string()
}

use std::sync::Arc;

use chrono::{DateTime, FixedOffset};
use leptos::{logging::log, prelude::*};

use crate::component::{
    modal::delete_event_modal::DeleteEventModal, time_util::calculate_time_pct,
};

use super::model::{Game, User};

/**
 * Display component for an event.
 */
#[component]
pub fn EventCard(
    title: String,
    selected_game: Option<Arc<Game>>,
    owner: Arc<User>,
    participants: Vec<Arc<User>>,
    suggestions: Vec<Arc<Game>>,
    start_time: DateTime<FixedOffset>,
    end_time: DateTime<FixedOffset>,
    baseline: DateTime<FixedOffset>,
    stacking_col: i32,
    session_id: i64,
    offset: usize,
) -> impl IntoView {
    let game_selected = selected_game.is_some();
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
                    <h2 class="text-xl font-bold">{ title }</h2>
                    // game title if game selected
                    {
                        selected_game.map(|g| view!{
                            <span>{ g.get_title() }</span>
                        })
                    }
                    // owner
                    <div class="flex flex-row gap-1">
                        <img
                            src={ owner.get_picture() }
                            alt={format!("{}'s profile picture", owner.get_name())}
                            class="size-8 shrink-0 rounded-full"
                        />
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
                        <div class="avatar avatar-placeholder border-primary border-2">
                            <div class="bg-neutral text-neutral-content w-8">
                                <span>{ format!("+{}", participants.len() - 1) }</span>
                            </div>
                        </div>
                    </div>

                        // suggestions if game not selected
                    <div id="suggestions-div">
                    {
                        (!game_selected).then(|| view! {
                            <h2 class="text-lg font-semibold">Suggestions</h2>
                            {
                                suggestions.iter()
                                    .map(|s| view! {<p class="font-sm">{ s.get_title() }</p>})
                                    .collect_view()
                            }
                        })
                    }
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

use std::sync::Arc;

use leptos::{
    html::{div, h1, img, p},
    prelude::*,
};

use super::model::{Game, User};

#[component]
pub fn EventCard(
    title: String,
    selected_game: Option<Arc<Game>>,
    owner: Arc<User>,
    participants: Vec<Arc<User>>,
    suggestions: Vec<Arc<Game>>,
) -> impl IntoView {
    let game_selected = selected_game.is_some();

    view! {
        <div class="flex w-48 flex-col rounded-xl p-4 border -outline-offset-1 outline-white/10">
            // event title and header
            <div id="header-box">
                // event title
                <h1 class = "text-xl font-large font-bold text-white">{ title }</h1>

                // game title if game selected
                {
                    selected_game.map(|g| view!{
                        <p class="text-gray-400 font-medium">{ g.get_title() }</p>
                    })
                }

                // owner
                <div class="flex flex-row gap-1">
                    <img
                        src={ owner.get_picture() }
                        alt={format!("{}'s profile picture", owner.get_name())}
                        class="size-8 shrink-0 rounded-full"
                        loading="eager"
                    />
                    <p class="font-medium">{ owner.get_name() }</p>
                </div>
            </div>

            // participants
            <div id="participants-div">
                <h2 class="text-lg font-semibold">Participants</h2>
                {
                    participants.iter()
                        .map(|p| view! {
                            <div class="flex flex-row gap-1">
                                <img
                                    src={ p.get_picture() }
                                    alt={format!("{}'s profile picture", p.get_name())}
                                    class="size-6 shrink-0 rounded-full"
                                    loading="eager"
                                />
                                <p class="font-sm">{ p.get_name() }</p>
                            </div>
                        })
                        .collect_view()
                }
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
    }
}

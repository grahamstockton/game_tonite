use std::sync::Arc;

use leptos::{logging::log, prelude::*};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes}, hooks::{use_params, use_params_map}, params::Params, path, StaticSegment
};
use leptos::Params;
use crate::component::{calendar::Calendar, event_card::EventCard, model::{Game, User}, user_profile_display::UserProfileDisplay};

#[derive(Params, PartialEq, Debug)]
struct SessionParams {
    group_id: String,
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="bg-gray-950">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/gaming-calendar-website.css"/>

        // sets the document title
        <Title text="Game Tonite!"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=path!(":id") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let params = use_params_map();
    let group_id = move || {
        params.read()
        .get("id")
        .unwrap_or_default()
    };

    view! {
        <div>
            /*<EventCard title={"Cheems Session".to_string()}
                selected_game={Some(Arc::new(Game {title: "cheemsgame".to_string(), cover_url: "".to_string()}))}
                owner={Arc::new(User {name: "graham".to_string(), picture: get_url()})}
                participants={vec![
                    Arc::new(User {name: "graham".to_string(), picture: get_url()}),
                    Arc::new(User {name: "jake".to_string(), picture: get_url()}),
                    Arc::new(User {name: "bob".to_string(), picture: get_url()})
                ]}
                suggestions={vec![Arc::new(Game {title: "cheemsgame".to_string(), cover_url: "".to_string()}), Arc::new(Game {title: "gomommor".to_string(), cover_url: "".to_string()})]}
            />
            <EventCard title={"Cheems Session".to_string()}
                selected_game={ None }
                owner={Arc::new(User {name: "graham".to_string(), picture: get_url()})}
                participants={vec![
                    Arc::new(User {name: "graham".to_string(), picture: get_url()}),
                    Arc::new(User {name: "jake".to_string(), picture: get_url()}),
                    Arc::new(User {name: "bob".to_string(), picture: get_url()})
                ]}
                suggestions={vec![Arc::new(Game {title: "cheemsgame".to_string(), cover_url: "".to_string()}), Arc::new(Game {title: "gomommor".to_string(), cover_url: "".to_string()})]}
            />*/
            <Calendar />
        </div>
    }
}


fn get_url() -> String {
    "https://wallpapers.com/images/featured/discord-profile-pictures-xk3qyllfj1j46kte.jpg".to_string()
}
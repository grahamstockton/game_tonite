use crate::component::calendar::Calendar;
use crate::component::navbar::NavBar;
use crate::obf_util::UrlParams;
use leptos::either::Either;
use leptos::prelude::*;
use leptos::Params;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::use_params_map,
    params::Params,
    path, StaticSegment,
};

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
            <body>
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
                    <Route path=StaticSegment("") view=WelcomePage/>
                    <Route path=path!(":id") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // parse params from url
    let params = use_params_map();
    let url_params = UrlParams::decode_url(params.read().get("id").unwrap_or_default());

    view! {
        <div class="relative">
        {
            match url_params {
                Ok(params) => {
                    Either::Left(view! {
                        <div class="relative z-4">
                            <NavBar />
                        </div>
                        <div class="relative z-0">
                            <Calendar url_params={params} />
                        </div>
                    })
                },
                Err(_) => {
                    Either::Right(view! {
                        <InvalidUrlPage />
                    })
                }
            }
        }
        </div>
    }
}

// Welcome page -- for routes without server_id, user_id
#[component]
fn WelcomePage() -> impl IntoView {
    view! {
        <NavBar />
        <p>Welcome to GameTonite</p>
    }
}

// Invalid url page
#[component]
fn InvalidUrlPage() -> impl IntoView {
    view! {
        <NavBar />
        <p>Invalid Url. Please Try Again.</p>
    }
}

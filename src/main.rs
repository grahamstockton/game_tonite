#[cfg(feature = "ssr")]
const DB_URL: &str = "sqlite://session.db";

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::sync::Arc;

    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use gaming_calendar_website::{app::*, dao::sqlite_util::SqliteClient};
    use gaming_calendar_website::dao::igdb_client::IgdbClient;
    use dotenv::dotenv;
    use sqlx::{migrate::MigrateDatabase as _, Sqlite, SqlitePool};

    // load env vars
    dotenv().ok();
    /*let client_id_str = std::env::var("CLIENT_ID").expect("couldn't find client id");
    let secret_val = std::env::var("SECRET").expect("couldn't find secret");

    // set up the http client
    let reqwest = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build().unwrap();

    let igdb = IgdbClient::new(Arc::new(reqwest), client_id_str, secret_val).await;

    igdb.get_games().await;*/

    // load sql client
    let sql = SqliteClient::new(DB_URL);

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}

#[cfg(feature = "ssr")]
use axum::extract::FromRef;
#[cfg(feature = "ssr")]
use leptos::config::LeptosOptions;
#[cfg(feature = "ssr")]
use sqlx::SqlitePool;

#[cfg(feature = "ssr")]
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: SqlitePool,
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use gaming_calendar_website::app::*;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::sqlite::SqlitePoolOptions;

    // load sql client
    //let sql = SqliteClient::new(DB_URL);*/
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    //setup db pool
    let pool = SqlitePoolOptions::new()
        .connect("sqlite://sessions.db")
        .await
        .expect("Could not make pool.");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("could not run SQLx migrations");

    let state = AppState {
        leptos_options: leptos_options,
        pool: pool,
    };

    let app = Router::new()
        .leptos_routes_with_context(
            &state,
            routes,
            {
                let pool = state.pool.clone();
                move || {
                    provide_context(pool.clone());
                }
            },
            {
                let state = state.clone();
                move || shell(state.leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler::<LeptosOptions, _>(
            shell,
        ))
        .with_state(state);

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

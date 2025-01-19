#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::sync::Arc;

    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use gaming_calendar_website::app::*;
    use gaming_calendar_website::dao::igdb_client::IgdbClient;
    use gaming_calendar_website::dao::sqlite_util::TestStruct;
    use dotenv::dotenv;
    use sqlx::SqlitePool;

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

    // load sqlite client
    // TODO: sort out how to get OpenSSL working for this
    let db = SqlitePool::connect("sqlite://session.db").await.unwrap();
    let result = sqlx::query("CREATE TABLE IF NOT EXISTS tests (id VARCHAR(250) PRIMARY KEY NOT NULL, val INTEGER NOT NULL);").execute(&db).await.unwrap();
    println!("Create user table result: {:?}", result); 
    let result = sqlx::query("INSERT INTO tests (name, val) VALUES (?, ?)").bind("harry").bind(20).execute(&db).await.unwrap();
    println!("Insert result: {:?}", result);
    let result = sqlx::query_as::<_, TestStruct>("SELECT id, name FROM users").fetch_all().await.unwrap();
    println!("Select result: {:?}", result);

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

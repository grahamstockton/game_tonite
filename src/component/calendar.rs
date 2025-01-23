use leptos::prelude::*;

#[component]
pub fn Calendar() -> impl IntoView {
    // we want this to reload every time there is an update

}

#[server]
fn get_events()
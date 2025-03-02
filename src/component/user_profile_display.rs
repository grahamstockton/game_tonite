use leptos::{
    html::{div, img, p},
    prelude::*,
};

#[component]
pub fn UserProfileDisplay(username: String, profile_url: String) -> impl IntoView {
    div()
        .class("mx-auto flex max-w-sm items-center gap-x-4 rounded-xl bg-white p-6 shadow-lg outline outline-black/5 dark:bg-slate-800 dark:shadow-none dark:-outline-offset-1 dark:outline-white/10")
        .child((
        img()
            .src(profile_url)
            .alt(format!("{}'s profile picture", username))
            .class("size-12 shrink-0 rounded-full"),
        p()
            .class("text-xl font-medium text-black dark:text-white")
            .child(username),
    ))
}

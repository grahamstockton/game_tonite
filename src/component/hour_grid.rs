use leptos::prelude::*;

/**
 * Hour grid. 24 divs, offset by a certain number of hours.
 * For example, if offset is 6, will start at 6am and end at 5am
 */
#[component]
pub fn HourGrid(offset: usize) -> impl IntoView {
    (0..24)
        .map(|h| {
            let v = (h + offset) % 24;
            view! {
                <div class="h-36 flex-shrink-0">
                    <hr class="z-0 border-contrast"/>
                    <p class="z-0 pl-2 text-contrast">{format!("{:0>2}:00", v)}</p>
                </div>
            }
        })
        .collect_view()
}

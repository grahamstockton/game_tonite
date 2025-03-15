use chrono::{DateTime, FixedOffset};
use leptos::prelude::*;

/**
 * Time overlay component. Takes in a time and a bottom padding and creates an overlay displaying that time.
 */
#[component]
pub fn TimeOverlay(
    bottom_pad_pct: ReadSignal<f64>,
    time: ReadSignal<DateTime<FixedOffset>>,
) -> impl IntoView {
    view! {
        <div class="absolute w-full flex-shrink-0" style={move || format!("bottom: {}%;", bottom_pad_pct()) }>
            <p class="text-sm pr-2 text-right z-2 text-accent">{ move || format!("{}", time().format("%H:%M")) }</p>
            <div class="z-2 divider divider-accent h-px m-0"></div>
        </div>
    }
}

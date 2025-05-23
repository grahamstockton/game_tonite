use chrono::{DateTime, FixedOffset};
use leptos::{html::Div, prelude::*};
use leptos_use::{
    use_element_size, use_interval_fn, use_scroll, use_window_size, UseElementSizeReturn,
    UseScrollReturn, UseWindowSizeReturn,
};

use crate::component::{
    calendar_events::CalendarEvents,
    hour_grid::HourGrid,
    time_overlay::TimeOverlay,
    time_util::{calculate_timebar_bottom, create_baseline, get_local_time},
};

#[component]
pub fn Calendar() -> impl IntoView {
    // some constants to do with displaying calendar
    const STARTING_HOUR_OFFSET: usize = 6;
    const SCROLL_OFFSET_PCT: f64 = 0.25;

    // get client side time
    let (time, set_time) =
        signal::<DateTime<FixedOffset>>(DateTime::from_timestamp(0, 0).unwrap().fixed_offset());
    let (timebar_bottom, set_timebar_bottom) = signal(0.);
    let (has_scrolled, set_has_scrolled) = signal(0);

    // client side baseline time. Updated only once
    let (baseline, set_baseline) = signal::<Option<DateTime<FixedOffset>>>(None);

    // node ref for scrolling
    let e = NodeRef::<Div>::new();
    let e2 = NodeRef::<Div>::new();
    let UseScrollReturn { set_y, .. } = use_scroll(e);
    let UseElementSizeReturn { height, .. } = use_element_size(e2);
    let UseWindowSizeReturn {
        height: window_height,
        ..
    } = use_window_size();

    // On render (client side) update time via effect.
    // Unfortunately, `use_interval_fn_with_options` initializes before the component renders
    // so this is necessary.
    Effect::new(move || {
        // set time locally
        let t = get_local_time();
        set_time(t);

        // set current time bar locally
        let tb = calculate_timebar_bottom(t, STARTING_HOUR_OFFSET);
        set_timebar_bottom(tb);

        // set baseline
        set_baseline(create_baseline(t, STARTING_HOUR_OFFSET).ok());
    });

    // Weird artifact of rendering, heigh begins at 0 and then is set to viewport height
    // Therefore, need this to run twice. Don't want it to run more though, so limit at 2.
    Effect::watch(
        move || height(),
        move |h, _, _| {
            // set screen scroll position
            if has_scrolled.get_untracked() < 2 {
                let sy = move || {
                    (100. - timebar_bottom.get_untracked()) / 100. * h
                        - SCROLL_OFFSET_PCT * window_height.get_untracked()
                };
                set_y(sy());
                *set_has_scrolled.write() += 1;
            }
        },
        false,
    );

    // Update time every 30s
    let _ = use_interval_fn(
        move || {
            let t = get_local_time();
            set_time(t);
            set_timebar_bottom(calculate_timebar_bottom(t, STARTING_HOUR_OFFSET));
        },
        30000,
    );

    view! {
        <div node_ref=e class="pt-16 z-0 relative flex flex-col h-dvh w-dvw overflow-y-scroll">
            <div node_ref=e2 class="relative flex-shrink-0">
                // foreground -- calendar events
                // ** time() without move || is intentional. Only want it once per load
                <CalendarEvents baseline={baseline} offset={STARTING_HOUR_OFFSET}/>

                // background -- hour grid
                <HourGrid offset={STARTING_HOUR_OFFSET}/>

                // overlay -- current time indicator
                <TimeOverlay bottom_pad_pct={timebar_bottom} time={time} />
            </div>
        </div>
    }
    .into_any()
}

use chrono::{DateTime, FixedOffset, TimeZone, Timelike, Utc};
use leptos::{html::Div, prelude::*};
use leptos_use::{use_element_size, use_interval_fn, use_scroll, use_window_size, UseElementSizeReturn, UseScrollReturn, UseWindowSizeReturn};

#[component]
pub fn Calendar() -> impl IntoView {
    // some constants to do with displaying calendar
    const STARTING_HOUR_OFFSET: usize = 6;
    const HORIZONTAL_LINE_OFFSET: &str = "1.25rem";
    const SCROLL_OFFSET_PCT: f64 = 0.25;

    // get client side time
    let (time, set_time) = signal::<DateTime<FixedOffset>>(DateTime::from_timestamp(0, 0).unwrap().fixed_offset());
    let (timebar_bottom, set_timebar_bottom) = signal(0.);

    // node ref for scrolling
    let e = NodeRef::<Div>::new();
    let e2 = NodeRef::<Div>::new();
    let UseScrollReturn { set_y, .. } = use_scroll(e);
    let UseElementSizeReturn { height, .. } = use_element_size(e2);
    let UseWindowSizeReturn { height: window_height, .. } = use_window_size();

    // On render (client side) update time via effect.
    // Unfortunately, `use_interval_fn_with_options` initializes before the component renders
    // so this is necessary.
    Effect::watch(
        move || height(),
        move |h, _, _| {
            // set time locally
            let t = get_local_time();
            set_time(t);

            // set current time bar locally
            let tb = calculate_timebar_bottom(t, STARTING_HOUR_OFFSET);
            set_timebar_bottom(tb);

            // set screen scroll position
            let sy = move || (100. - tb) / 100. * h - SCROLL_OFFSET_PCT * window_height.get_untracked();
            set_y(sy());
        },
        false,
    );

    // Update time every 30s
    let _ = use_interval_fn(
        move || {
            set_time(get_local_time());
            set_timebar_bottom(calculate_timebar_bottom(time(), STARTING_HOUR_OFFSET));
        },
        30000,
    );

    view! {
        <div node_ref=e class="relative flex flex-col h-dvh w-dvw bg-slate-950 overflow-y-scroll">
            <div node_ref=e2 class="relative flex-shrink-0">
                // background -- hour grid
                {
                    (0..24).map(|h| {
                        let v = (h + STARTING_HOUR_OFFSET) % 24;
                        view! {
                            <div class="h-24 flex-shrink-0">
                                <hr class="z-0 border-gray-400"/>
                                <p class="z-0 pl-2 text-gray-400">{format!("{:0>2}:00", v)}</p>
                            </div>
                        }
                    }).collect_view()
                }
                // foreground layer -- event components
                // overlay -- current time indicator
                <div class="absolute w-full flex-shrink-0" style={move || format!("bottom: {}%;", timebar_bottom()) }>
                    <p class="text-sm pr-2 text-right z-2 text-fuchsia-700">{ move || format!("{}", time().format("%H:%M")) }</p>
                    <hr class="border z-2 w-full border-fuchsia-700"/>
                </div>
            </div>
        </div>
    }
}

// Given a date and a number of hours offset for the display, return bottom padding
fn calculate_timebar_bottom(t: DateTime<FixedOffset>, offset: usize) -> f64 {
    let nsfm = t.num_seconds_from_midnight() as f64;
    let offset_secs = offset as f64 * 3600.;

    if nsfm < offset_secs {
        100. * (offset_secs - nsfm) / 86400.
    } else {
        100. * (1. - (nsfm - offset_secs) / 86400.)
    }
}

// get the local time with timezone from the client
fn get_local_time() -> DateTime<FixedOffset> {
    let mins_offset = js_sys::Date::new_0().get_timezone_offset();
    let offset = FixedOffset::west_opt((mins_offset * 60.) as i32).unwrap();
    
    offset.from_utc_datetime(&Utc::now().naive_utc())
} 
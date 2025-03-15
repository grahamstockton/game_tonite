use chrono::{DateTime, FixedOffset, TimeZone, Timelike, Utc};

// get the local time with timezone from the client
pub fn get_local_time() -> DateTime<FixedOffset> {
    let mins_offset = js_sys::Date::new_0().get_timezone_offset();
    let offset = FixedOffset::west_opt((mins_offset * 60.) as i32).unwrap();

    offset.from_utc_datetime(&Utc::now().naive_utc())
}

// Given a date and a number of hours offset for the display, return bottom padding
pub fn calculate_timebar_bottom(t: DateTime<FixedOffset>, offset: usize) -> f64 {
    let nsfm = t.num_seconds_from_midnight() as f64;
    let offset_secs = offset as f64 * 3600.;

    if nsfm < offset_secs {
        100. * (offset_secs - nsfm) / 86400.
    } else {
        100. * (1. - (nsfm - offset_secs) / 86400.)
    }
}

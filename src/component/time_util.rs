use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Days, Duration, FixedOffset, SubsecRound, TimeZone, Timelike, Utc};

use super::model::GamingSession;

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

/**
 * Get the percent throughout the timeline, with offset.
 * Baseline time should be midnight on the day the offset is applied to.
 * Limited between 0 and 1.
 */
pub fn calculate_time_pct(
    time: DateTime<FixedOffset>,
    baseline: DateTime<FixedOffset>,
    offset: usize,
) -> f64 {
    let pct = (time - baseline).num_seconds() as f64 / 86400. - (offset as f64) / 24.;
    if pct > 1. {
        1.
    } else if pct < 0. {
        0.
    } else {
        pct
    }
}

/**
 * Create the baseline time from a time and offset
 * If time < offset: return beginning of previous day
 * If time >= offset: return beginning of current day
 */
pub fn create_baseline(
    time: DateTime<FixedOffset>,
    offset: usize,
) -> Result<DateTime<FixedOffset>> {
    let seconds_from_midnight = time.num_seconds_from_midnight();
    if seconds_from_midnight < offset as u32 * 3600 {
        Ok(time - Duration::seconds((86400 + seconds_from_midnight) as i64))
    } else {
        Ok(time - Duration::seconds(seconds_from_midnight as i64))
    }
}

/**
 * Stack elements in horizontal space so they don't overlap
 * Returns a HashMap of session_id to positioning. Positioning starts at 0.
 */
pub fn get_events_stacking(events: &Vec<GamingSession>) -> HashMap<i64, i32> {
    // sort by starting time
    let mut sorted_events = events.clone();
    sorted_events.sort_by_key(|e| e.start_time);

    // map from position to ending time -- to check if a new event fits in position
    let mut latest_times: HashMap<i32, DateTime<Utc>> = HashMap::new();

    /*
     * For each event:
     *  - Check if it fits in position 0
     *  - If not, increment position and try again
     */
    let mut return_map: HashMap<i64, i32> = HashMap::new();
    for event in sorted_events {
        let mut position_index = 0;
        let mut is_complete: bool = false;
        while !is_complete {
            let _ = match latest_times.get(&position_index) {
                // no events exist in position, good to insert
                None => {
                    latest_times.insert(position_index.clone(), event.end_time);
                    return_map.insert(event.session_id, position_index.clone());
                    is_complete = true;
                }
                // check if event fits (start time is after latest existing end time)
                Some(t) => {
                    if t <= &event.start_time {
                        latest_times.insert(position_index.clone(), event.end_time);
                        return_map.insert(event.session_id, position_index.clone());
                        is_complete = true;
                    } else {
                        is_complete = false;
                    }
                }
            };
            // try next position
            position_index += 1;
        }
    }

    return_map
}

#[cfg(test)]
mod tests {
    use crate::component::{
        model::{GamingSession, User},
        time_util::get_events_stacking,
    };
    use chrono::{DateTime, Utc};
    use std::collections::HashMap;

    struct Setup {
        time_1: DateTime<Utc>,
        time_2: DateTime<Utc>,
        time_3: DateTime<Utc>,
        time_4: DateTime<Utc>,
        server_id: String,
        session_id_1: i64,
        session_id_2: i64,
        session_id_3: i64,
        title: String,
        owner: User,
        participants: Vec<User>,
    }

    impl Setup {
        fn new() -> Self {
            Self {
                time_1: DateTime::parse_from_rfc3339("1996-12-19T16:00:00Z")
                    .unwrap()
                    .into(),
                time_2: DateTime::parse_from_rfc3339("1996-12-19T17:00:00Z")
                    .unwrap()
                    .into(),
                time_3: DateTime::parse_from_rfc3339("1996-12-19T18:00:00Z")
                    .unwrap()
                    .into(),
                time_4: DateTime::parse_from_rfc3339("1996-12-19T19:00:00Z")
                    .unwrap()
                    .into(),
                server_id: "server_id".to_string(),
                session_id_1: 111111111111,
                session_id_2: 222222222222,
                session_id_3: 333333333333,
                title: "title".to_string(),
                owner: User {
                    name: "username".to_string(),
                    picture: "picture".to_string(),
                },
                participants: vec![],
            }
        }
    }

    fn create_gaming_session(
        session_id: &i64,
        start_time: &DateTime<Utc>,
        end_time: &DateTime<Utc>,
    ) -> GamingSession {
        let setup = Setup::new();
        GamingSession {
            server_id: setup.server_id,
            session_id: session_id.clone(),
            title: setup.title,
            start_time: start_time.clone(),
            end_time: end_time.clone(),
            owner: setup.owner,
            participants: setup.participants,
        }
    }

    #[test]
    fn test_empty_case() {
        let input: Vec<GamingSession> = vec![];
        let expected: HashMap<i64, i32> = HashMap::new();
        let res = get_events_stacking(&input);
        assert_eq!(expected, res);
    }

    #[test]
    fn test_stacking_case() {
        let setup = Setup::new();
        let input: Vec<GamingSession> = vec![
            create_gaming_session(&setup.session_id_1, &setup.time_1, &setup.time_3),
            create_gaming_session(&setup.session_id_2, &setup.time_2, &setup.time_4),
        ];
        let expected: HashMap<i64, i32> =
            HashMap::from([(setup.session_id_1, 0), (setup.session_id_2, 1)]);
        let res = get_events_stacking(&input);
        assert_eq!(expected, res);
    }

    #[test]
    fn test_end_equals_start() {
        let setup = Setup::new();
        let input: Vec<GamingSession> = vec![
            create_gaming_session(&setup.session_id_1, &setup.time_1, &setup.time_2),
            create_gaming_session(&setup.session_id_2, &setup.time_2, &setup.time_3),
        ];
        let expected: HashMap<i64, i32> =
            HashMap::from([(setup.session_id_1, 0), (setup.session_id_2, 0)]);
        let res = get_events_stacking(&input);
        assert_eq!(expected, res);
    }

    #[test]
    fn test_end_equals_and_overlap() {
        let setup = Setup::new();
        let input: Vec<GamingSession> = vec![
            create_gaming_session(&setup.session_id_1, &setup.time_1, &setup.time_3),
            create_gaming_session(&setup.session_id_2, &setup.time_2, &setup.time_4),
            create_gaming_session(&setup.session_id_3, &setup.time_3, &setup.time_4),
        ];
        let expected: HashMap<i64, i32> = HashMap::from([
            (setup.session_id_1, 0),
            (setup.session_id_2, 1),
            (setup.session_id_3, 0),
        ]);
        let res = get_events_stacking(&input);
        assert_eq!(expected, res);
    }

    #[test]
    fn test_second_third_event_same() {
        let setup = Setup::new();
        let input: Vec<GamingSession> = vec![
            create_gaming_session(&setup.session_id_1, &setup.time_1, &setup.time_3),
            create_gaming_session(&setup.session_id_2, &setup.time_3, &setup.time_4),
            create_gaming_session(&setup.session_id_3, &setup.time_3, &setup.time_4),
        ];
        let expected: HashMap<i64, i32> = HashMap::from([
            (setup.session_id_1, 0),
            (setup.session_id_2, 0),
            (setup.session_id_3, 1),
        ]);
        let res = get_events_stacking(&input);
        assert_eq!(expected, res);
    }
}

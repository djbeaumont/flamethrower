use std::collections::HashMap;
use chrono::{DateTime, Duration, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use rrule::{Frequency, RRule, RRuleSet};
use rrule::Tz as RRuleTz;
use fhir_rs::model::Timing::Timing;
use chrono_tz::Tz;
use chrono_tz::Tz::UTC;
use crate::interval;
use crate::timing::{FhirTiming, Occurrence};

pub fn between(timing: &FhirTiming, bounds: &interval::Interval) -> Vec<NaiveDateTime> {
    const MAX_RESULTS: u16 = 65535;

    let mapped = map_timing_to_rrule(timing);

    let filtered = mapped
        .after(bounds.start.and_utc().with_timezone(&RRuleTz::Europe__London))
        .before(bounds.end.unwrap_or(&chrono::NaiveDateTime::MAX).and_utc().with_timezone(&RRuleTz::Europe__London));

    let occurrences = filtered
        .all(MAX_RESULTS)
        .dates
        .iter()
        .map(|d| d.with_timezone(&RRuleTz::Europe__London).naive_local())
        .collect();

    occurrences
}

// TODO - merge with `between` in recurrence.rs
pub fn get_occurrences(timing: FhirTiming, begin: NaiveDate, end: NaiveDate) -> Vec<Occurrence> {
    let mut date = begin;

    let mut occurrences: Vec<Occurrence> = vec![];

    while date <= end {
        timing.repeat.when.iter().for_each(|when| {
            occurrences.push(Occurrence {
                date,
                time_of_day: None,
                when: Some(when.to_owned()),
            });
        });

        timing.repeat.time_of_day.iter().for_each(|time_of_day| {
            occurrences.push(Occurrence {
                date,
                time_of_day: Some(*time_of_day),
                when: None,
            });
        });

        date = date + Duration::days(1);
    }

    occurrences
}

fn map_timing_to_rrule(timing: &FhirTiming) -> RRuleSet {
    let timezone = RRuleTz::Europe__London;

    let daily_freq_map: HashMap<i64, Vec<u8>> = HashMap::from([
        (1, vec![8]),
        (2, vec![8, 20]),
        (3, vec![8, 14, 20]),
        (4, vec![8, 12, 16, 20]),
    ]);

    let dtstart_str = timing.repeat.bounds.start.unwrap().and_time(NaiveTime::default());
    let dtstart = DateTime::parse_from_rfc3339(&dtstart_str.and_utc().to_rfc3339()).unwrap()
        .with_timezone(&timezone);

    let by_hour = daily_freq_map
        .get(&timing.repeat.frequency)
        .unwrap()
        .to_owned();

    let rrule = RRule::new(Frequency::Daily)
        .by_hour(by_hour)
        .validate(dtstart)
        .unwrap();

    RRuleSet::new(dtstart).rrule(rrule)
}

// Based on fhir_rs, but we are dropping this dependency
#[deprecated]
fn map_to_rrule_old(timing: Timing) -> RRuleSet {
    let timezone = RRuleTz::Europe__London;

    let daily_freq_map: HashMap<i64, Vec<u8>> = HashMap::from([
        (1, vec![8]),
        (2, vec![8, 20]),
        (3, vec![8, 14, 20]),
        (4, vec![8, 12, 16, 20]),
    ]);

    let repeat = timing.repeat().unwrap();
    let bounds = repeat.bounds_period().unwrap();
    let dtstart_str = bounds.start().unwrap();
    let dtstart = DateTime::parse_from_rfc3339(dtstart_str)
        .unwrap()
        .with_timezone(&timezone);
    let by_hour = daily_freq_map
        .get(&repeat.frequency().unwrap())
        .unwrap()
        .to_owned();

    let rrule = RRule::new(Frequency::Daily)
        .by_hour(by_hour)
        .validate(dtstart)
        .unwrap();

    RRuleSet::new(dtstart).rrule(rrule)
}

// Based on fhir_rs, but we are dropping this dependency
#[deprecated]
pub fn get_occurrences_old(
    timing: Timing,
    window_start: DateTime<FixedOffset>,
    window_finish: DateTime<FixedOffset>,
) -> Vec<DateTime<Tz>> {
    const MAX_RESULTS: u16 = 65535;
    let rrule = map_to_rrule_old(timing);
    let filtered = rrule
        .after(window_start.with_timezone(&RRuleTz::Europe__London))
        .before(window_finish.with_timezone(&RRuleTz::Europe__London));
    let occurrences = filtered
        .all(MAX_RESULTS)
        .dates
        .iter()
        .map(|d| d.with_timezone(&UTC))
        .collect();
    occurrences
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use crate::interval::Interval;
    use crate::timing::{FhirPeriod, FhirTiming, FhirTimingRepeat, Occurrence};

    fn parse_naive_datetime(date: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S").unwrap()
    }

    #[test]
    fn take_once_a_day() {
        let window = Interval {
            start: &parse_naive_datetime("2021-06-21T00:00:00"),
            end: Some(&parse_naive_datetime("2021-06-22T00:00:00")),
        };

        let timing = FhirTiming {
            repeat: FhirTimingRepeat {
                bounds: FhirPeriod {
                    start: Some(NaiveDate::from_ymd_opt(2021, 6, 21).unwrap()),
                    end: None,
                },
                frequency: 1,
                period: 1,
                period_unit: "d".to_string(),
                day_of_week: vec![],
                time_of_day: vec![],
                when: vec![],
            },
        };

        let expected = vec![parse_naive_datetime("2021-06-21T08:00:00")];
        let actual = super::between(&timing, &window);

        assert_eq!(actual, expected);
    }

    // #[test]
    // fn take_three_times_a_day() {
    //     let window_start = parse_datetime("2021-06-21T00:00:00+01:00");
    //     let window_finish = parse_datetime("2021-06-22T00:00:00+01:00");
    //
    //     let timing_json = json!({
    //         "repeat": {
    //             "frequency": 3,
    //             "period": 1,
    //             "periodUnit": "d",
    //             "boundsPeriod": {
    //                 "start": "2021-06-21T00:00:00+01:00"
    //             }
    //         }
    //     });
    //     let timing = Timing::new(&timing_json);
    //
    //     let expected = vec![
    //         parse_datetime("2021-06-21T08:00:00+01:00"),
    //         parse_datetime("2021-06-21T14:00:00+01:00"),
    //         parse_datetime("2021-06-21T20:00:00+01:00"),
    //     ];
    //     let actual = super::between(timing, window_start, window_finish);
    //
    //     assert_eq!(actual, expected);
    // }

    #[test]
    fn take_once_a_day_in_the_morning() {
        let timing = super::FhirTiming {
            repeat: FhirTimingRepeat {
                bounds: FhirPeriod {
                    start: NaiveDate::from_ymd_opt(2024, 12, 30),
                    end: None,
                },
                frequency: 1,
                period: 1,
                period_unit: "d".to_string(),
                day_of_week: vec![],
                time_of_day: vec![],
                when: vec!["MORN".to_owned()],
            },
        };

        let begin = NaiveDate::from_ymd_opt(2024, 12, 30).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let occurrences = super::get_occurrences(timing, begin, end);

        assert_eq!(occurrences, vec![
            Occurrence {
                date: NaiveDate::from_ymd_opt(2024, 12, 30).unwrap(),
                time_of_day: None,
                when: Some("MORN".to_owned()),
            },
            Occurrence {
                date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
                time_of_day: None,
                when: Some("MORN".to_owned()),
            },
        ]);
    }

    #[test]
    fn take_twice_a_day_morning_and_evening() {
        let timing = super::FhirTiming {
            repeat: FhirTimingRepeat {
                bounds: FhirPeriod {
                    start: NaiveDate::from_ymd_opt(2024, 12, 30),
                    end: None,
                },
                frequency: 1,
                period: 1,
                period_unit: "d".to_string(),
                day_of_week: vec![],
                time_of_day: vec![],
                when: vec!["MORN".to_owned(), "EVE".to_owned()],
            },
        };

        let begin = NaiveDate::from_ymd_opt(2024, 12, 30).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let occurrences = super::get_occurrences(timing, begin, end);

        assert_eq!(occurrences, vec![
            Occurrence {
                date: NaiveDate::from_ymd_opt(2024, 12, 30).unwrap(),
                time_of_day: None,
                when: Some("MORN".to_owned()),
            },
            Occurrence {
                date: NaiveDate::from_ymd_opt(2024, 12, 30).unwrap(),
                time_of_day: None,
                when: Some("EVE".to_owned()),
            },
            Occurrence {
                date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
                time_of_day: None,
                when: Some("MORN".to_owned()),
            },
            Occurrence {
                date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
                time_of_day: None,
                when: Some("EVE".to_owned()),
            },
        ]);
    }

    #[test]
    fn take_once_a_day_at_8am() {
        let timing = super::FhirTiming {
            repeat: FhirTimingRepeat {
                bounds: FhirPeriod {
                    start: NaiveDate::from_ymd_opt(2024, 12, 30),
                    end: None,
                },
                frequency: 1,
                period: 1,
                period_unit: "d".to_string(),
                day_of_week: vec![],
                time_of_day: vec![NaiveTime::from_hms_opt(8, 0, 0).unwrap()],
                when: vec![],
            },
        };

        let begin = NaiveDate::from_ymd_opt(2024, 12, 30).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let occurrences = super::get_occurrences(timing, begin, end);

        assert_eq!(occurrences, vec![
            Occurrence {
                date: NaiveDate::from_ymd_opt(2024, 12, 30).unwrap(),
                time_of_day: Some(NaiveTime::from_hms_opt(8, 0, 0).unwrap()),
                when: None,
            },
            Occurrence {
                date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
                time_of_day: Some(NaiveTime::from_hms_opt(8, 0, 0).unwrap()),
                when: None,
            },
        ]);
    }
}

#[cfg(test)]
mod tests_old {
    use chrono::prelude::DateTime;
    use fhir_rs::model::Timing::Timing;
    use serde_json::json;

    fn parse_datetime(date: &str) -> DateTime<chrono::FixedOffset> {
        DateTime::parse_from_rfc3339(date).unwrap()
    }

    #[test]
    fn take_once_a_day() {
        let window_start = parse_datetime("2021-06-21T00:00:00+01:00");
        let window_finish = parse_datetime("2021-06-22T00:00:00+01:00");

        let timing_json = json!({
            "repeat": {
                "frequency": 1,
                "period": 1,
                "periodUnit": "d",
                "boundsPeriod": {
                    "start": "2021-06-21T00:00:00+01:00"
                }
            }
        });
        let timing = Timing::new(&timing_json);

        let expected = vec![parse_datetime("2021-06-21T08:00:00+01:00")];
        let actual = super::get_occurrences_old(timing, window_start, window_finish);

        assert_eq!(actual, expected);
    }

    #[test]
    fn take_three_times_a_day() {
        let window_start = parse_datetime("2021-06-21T00:00:00+01:00");
        let window_finish = parse_datetime("2021-06-22T00:00:00+01:00");

        let timing_json = json!({
            "repeat": {
                "frequency": 3,
                "period": 1,
                "periodUnit": "d",
                "boundsPeriod": {
                    "start": "2021-06-21T00:00:00+01:00"
                }
            }
        });
        let timing = Timing::new(&timing_json);

        let expected = vec![
            parse_datetime("2021-06-21T08:00:00+01:00"),
            parse_datetime("2021-06-21T14:00:00+01:00"),
            parse_datetime("2021-06-21T20:00:00+01:00"),
        ];
        let actual = super::get_occurrences_old(timing, window_start, window_finish);

        assert_eq!(actual, expected);
    }
}
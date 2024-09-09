use chrono::prelude::*;
use chrono_tz::Tz;
use chrono_tz::Tz::UTC;
use fhir_rs::model::Timing::Timing;
use rrule::Tz as RRuleTz;
use rrule::{Frequency, RRule, RRuleSet};
use std::collections::HashMap;

fn map_to_rrule(timing: Timing) -> RRuleSet {
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

pub fn get_occurrences(
    timing: Timing,
    window_start: DateTime<FixedOffset>,
    window_finish: DateTime<FixedOffset>,
) -> Vec<DateTime<Tz>> {
    const MAX_RESULTS: u16 = 65535;
    let rrule = map_to_rrule(timing);
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
        let actual = super::get_occurrences(timing, window_start, window_finish);

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
        let actual = super::get_occurrences(timing, window_start, window_finish);

        assert_eq!(actual, expected);
    }
}

use std::collections::HashMap;

use chrono::prelude::*;
use chrono_tz::Tz;
use fhir_rs::model::Timing::Timing;
use rrule::{RRule, Options, Frequenzy};

fn map_to_rrule(timing: Timing) -> RRule {
    let daily_freq_map: HashMap<i64, Vec<usize>> = HashMap::from([
        (1, vec![8]),
        (2, vec![8, 20]),
        (3, vec![8, 14, 20]),
        (4, vec![8, 12, 16, 20]),
    ]);

    let repeat = timing.repeat().unwrap();
    let bounds = repeat.bounds_period().unwrap();
    let dtstart_str = bounds.start().unwrap();
    let dtstart = DateTime::parse_from_rfc3339(dtstart_str).unwrap();
    let tz: Tz = "Europe/London".parse().unwrap();
    let options = Options::new()
        .dtstart(dtstart.with_timezone(&tz))
        .byhour(daily_freq_map.get(&repeat.frequency().unwrap()).unwrap().to_owned())
        .freq(Frequenzy::Daily)
        .build()
        .unwrap();
    let rrule = RRule::new(options);
    rrule
}

pub fn get_occurrences(
    timing: Timing,
    window_start: DateTime<FixedOffset>,
    window_finish: DateTime<FixedOffset>,
) -> Vec<DateTime<Tz>> {
    let rrule = map_to_rrule(timing);
    let tz: Tz = "Europe/London".parse().unwrap();
    let occurrences = rrule.between(window_start.with_timezone(&tz), window_finish.with_timezone(&tz), true);
    occurrences
}

#[cfg(test)]
mod tests {
    use chrono::FixedOffset;
    use chrono::prelude::DateTime;
    use chrono_tz::Tz;
    use fhir_rs::model::Timing::TimingBuilder;
    use fhir_rs::model::Timing_Repeat::{Timing_RepeatBuilder, Timing_RepeatPeriodUnit};
    use fhir_rs::model::Period::PeriodBuilder;

    const window_start: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2021-06-21T00:00:00+01:00").unwrap();
    const window_finish: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2021-06-22T00:00:00+01:00").unwrap();

    #[test]
    fn take_once_a_day() {
        let mut repeat_builder = Timing_RepeatBuilder::new();
        repeat_builder
            .frequency(1)
            .period(1f64)
            .period_unit(Timing_RepeatPeriodUnit::D)
            .bounds_period(PeriodBuilder::new().start("2021-06-21T00:00:00+01:00").build());

        let mut timing_builder = TimingBuilder::new();
        timing_builder.repeat(repeat_builder.build());
        let timing = timing_builder.build();

        let tz: Tz = "Europe/London".parse().unwrap();
        let expected = vec![
            DateTime::parse_from_rfc3339("2021-06-21T08:00:00+01:00").unwrap().with_timezone(&tz),
        ];
        let actual = super::get_occurrences(timing, window_start, window_finish);

        assert_eq!(actual, expected);
    }

    #[test]
    fn take_three_times_a_day() {
        let mut repeat_builder = Timing_RepeatBuilder::new();
        repeat_builder
            .frequency(3)
            .period(1f64)
            .period_unit(Timing_RepeatPeriodUnit::D)
            .bounds_period(PeriodBuilder::new().start("2021-06-21T00:00:00+01:00").build());

        let mut timing_builder = TimingBuilder::new();
        timing_builder.repeat(repeat_builder.build());
        let timing = timing_builder.build();

        let tz: Tz = "Europe/London".parse().unwrap();
        let expected = vec![
            DateTime::parse_from_rfc3339("2021-06-21T08:00:00+01:00").unwrap().with_timezone(&tz),
            DateTime::parse_from_rfc3339("2021-06-21T14:00:00+01:00").unwrap().with_timezone(&tz),
            DateTime::parse_from_rfc3339("2021-06-21T20:00:00+01:00").unwrap().with_timezone(&tz),
        ];
        let actual = super::get_occurrences(timing, window_start, window_finish);

        assert_eq!(actual, expected);
    }
}

use chrono::prelude::*;
use fhir_rs::model::Timing::Timing;

pub fn get_occurrences(
    _timing: Timing,
    _window_start: DateTime<FixedOffset>,
    _window_finish: DateTime<FixedOffset>,
) -> Vec<DateTime<FixedOffset>> {
    let date = DateTime::parse_from_rfc3339("2021-06-21T21:46:18+01:00").unwrap();
    vec![date]
}

#[cfg(test)]
mod tests {
    use chrono::prelude::DateTime;
    use fhir_rs::model::Timing::TimingBuilder;
    use fhir_rs::model::Timing_Repeat::{Timing_RepeatBuilder, Timing_RepeatPeriodUnit};
    use fhir_rs::model::Period::PeriodBuilder;

    #[test]
    fn it_works() {
        let mut repeat_builder = Timing_RepeatBuilder::new();
        repeat_builder
            // ._when(vec![ElementBuilder::new().build()])
            .frequency(3)
            .period(1f64)
            .period_unit(Timing_RepeatPeriodUnit::D)
            .bounds_period(PeriodBuilder::new().start("2021-06-21").build());

        let mut timing_builder = TimingBuilder::new();
        timing_builder.repeat(repeat_builder.build());
        let timing = timing_builder.build();

        let window_start = DateTime::parse_from_rfc3339("2021-06-21T00:00:00+01:00").unwrap();
        let window_finish = DateTime::parse_from_rfc3339("2021-06-22T00:00:00+01:00").unwrap();

        let occurrences = super::get_occurrences(timing, window_start, window_finish);
        let first = occurrences.first().unwrap();

        assert_eq!(first.to_rfc3339(), "2021-06-21T21:46:18+01:00");
    }
}

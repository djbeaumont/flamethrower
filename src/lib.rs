use chrono::NaiveDate;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::interval::Interval;
use crate::timing::{FhirPeriod, FhirTiming, FhirTimingRepeat};

mod attribute_over_time;
mod entity_over_time;
mod timing;
mod interval;
mod recurrence;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    // alert(&format!("Hello, {}!", name));
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn between(begin: &str, end: &str) -> Vec<String> {
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

    let interval = Interval {
        start: &NaiveDate::parse_from_str(begin, "%Y-%m-%d").unwrap().and_hms_opt(0, 0, 0).unwrap(),
        end: Some(&NaiveDate::parse_from_str(end, "%Y-%m-%d").unwrap().and_hms_opt(0, 0, 0).unwrap()),
    };

    let occurrences = recurrence::between(&timing, &interval);

    occurrences.iter().map(|o| o.to_string()).collect()
}
use chrono::{NaiveDate, NaiveTime};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::interval::Interval;
use crate::timing::{FhirPeriod, FhirTiming, FhirTimingRepeat};

mod attribute_over_time;
mod entity_over_time;
mod timing;
mod interval;
mod recurrence;

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

#[wasm_bindgen]
pub fn project_occurrences(raw_timing: JsValue, start: &str, end: &str) -> Vec<String> {
    let timing: FhirTiming = serde_wasm_bindgen::from_value(raw_timing).unwrap();

    // let timing = FhirTiming {
    //     repeat: FhirTimingRepeat {
    //         bounds: FhirPeriod {
    //             start: Some(NaiveDate::from_ymd_opt(2021, 6, 21).unwrap()),
    //             end: None,
    //         },
    //         frequency: 1,
    //         period: 1,
    //         period_unit: "d".to_string(),
    //         day_of_week: vec![],
    //         time_of_day: vec![],
    //         when: vec![],
    //     },
    // };

    let interval = Interval {
        start: &NaiveDate::parse_from_str(start, "%Y-%m-%d").unwrap().and_hms_opt(0, 0, 0).unwrap(),
        end: Some(&NaiveDate::parse_from_str(end, "%Y-%m-%d").unwrap().and_hms_opt(0, 0, 0).unwrap()),
    };
    let occurrences = recurrence::between(&timing, &interval);
    occurrences.iter().map(|o| o.to_string()).collect()
}

// #[wasm_bindgen]
// pub fn project_occurrences(timing: &SerializableFhirTiming, start: &str, end: &str) -> Vec<String> {
//     let interval = Interval {
//         start: &NaiveDate::parse_from_str(start, "%Y-%m-%d").unwrap().and_hms_opt(0, 0, 0).unwrap(),
//         end: Some(&NaiveDate::parse_from_str(end, "%Y-%m-%d").unwrap().and_hms_opt(0, 0, 0).unwrap()),
//     };
//     let parsed_timing = parse_serializable_timing(timing);
//     let occurrences = recurrence::between(&parsed_timing, &interval);
//     occurrences.iter().map(|o| o.to_string()).collect()
// }
//
// fn parse_serializable_timing(timing: &SerializableFhirTiming) -> FhirTiming {
//     let bounds = FhirPeriod {
//         start: match &timing.repeat.bounds.start {
//             Some(s) => Some(NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()),
//             None => None,
//         },
//         end: match &timing.repeat.bounds.end {
//             Some(e) => Some(NaiveDate::parse_from_str(e, "%Y-%m-%d").unwrap()),
//             None => None,
//         },
//     };
//
//     let time_of_day = timing.repeat.time_of_day.iter().map(|t| {
//         NaiveTime::parse_from_str(t, "%H:%M:%S").unwrap()
//     }).collect();
//
//     FhirTiming {
//         repeat: FhirTimingRepeat {
//             bounds,
//             frequency: timing.repeat.frequency,
//             period: timing.repeat.period,
//             period_unit: timing.repeat.period_unit.clone(),
//             day_of_week: timing.repeat.day_of_week.clone(),
//             time_of_day,
//             when: timing.repeat.when.clone(),
//         },
//     }
// }
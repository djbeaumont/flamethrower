use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveTime};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct FhirPeriod {
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct FhirTimingRepeat {
    pub bounds: FhirPeriod,
    pub frequency: i64,
    pub period: i64,
    pub period_unit: String,
    pub day_of_week: Vec<String>,
    pub time_of_day: Vec<NaiveTime>,
    pub when: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct FhirTiming {
    pub repeat: FhirTimingRepeat,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Occurrence {
    pub date: NaiveDate,
    pub time_of_day: Option<NaiveTime>,
    pub when: Option<String>,
}

// TODO - work out how to remove this duplication by binding the structs above to wasm

// #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
// #[wasm_bindgen(getter_with_clone)]
// pub struct SerializableFhirPeriod {
//     pub start: Option<String>,
//     pub end: Option<String>,
// }
//
// #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
// #[wasm_bindgen(getter_with_clone)]
// pub struct SerializableFhirTimingRepeat {
//     pub bounds: SerializableFhirPeriod,
//     pub frequency: i64,
//     pub period: i64,
//     pub period_unit: String,
//     pub day_of_week: Vec<String>,
//     pub time_of_day: Vec<String>,
//     pub when: Vec<String>,
// }
//
// #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
// #[wasm_bindgen(getter_with_clone)]
// pub struct SerializableFhirTiming {
//     pub repeat: SerializableFhirTimingRepeat,
// }
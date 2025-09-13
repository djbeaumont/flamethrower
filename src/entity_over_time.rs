use std::collections::HashMap;
use crate::attribute_over_time::AttributeOverTime;
use crate::interval::Interval;
use crate::timing::FhirTiming;

pub struct EntityOccurrence<'a> {
    attributes: HashMap<String, &'a String>,
    timing: &'a FhirTiming,
}

pub struct EntityOverTime<'a> {
    // FIXME - we need more flexibility than just String attributes
    attributes: HashMap<String, AttributeOverTime<'a, String>>,
    timing: AttributeOverTime<'a, FhirTiming>,
}

impl<'a> EntityOverTime<'a> {
    pub fn new() -> Self {
        EntityOverTime {
            attributes: HashMap::new(),
            timing: AttributeOverTime::new(),
        }
    }

    pub fn new_with_initial_state(
        attributes: HashMap<String, &'a String>,
        timing: &'a FhirTiming,
        effective_from: &'a chrono::NaiveDateTime,
    ) -> Self {
        let mut attributes_over_time: HashMap<String, AttributeOverTime<'a, String>> = HashMap::new();

        for (name, value) in attributes {
            let attribute_over_time = AttributeOverTime::new_with_value(effective_from, value);
            attributes_over_time.insert(name, attribute_over_time);
        }

        let timing_over_time = AttributeOverTime::new_with_value(effective_from, timing);

        EntityOverTime {
            attributes: attributes_over_time,
            timing: timing_over_time,
        }
    }

    pub fn with_attribute_value(&self, name: String, value: &'a String, effective_from: &'a chrono::NaiveDateTime) -> Self {
        let mut updated_attributes: HashMap<String, AttributeOverTime<'a, String>> = self.attributes.clone();

        let attribute_over_time = if let Some(existing) = updated_attributes.get_mut(&name) {
            existing.with_value(effective_from, value)
        } else {
            AttributeOverTime::new_with_value(effective_from, value)
        };

        updated_attributes.insert(name, attribute_over_time);

        EntityOverTime {
            attributes: updated_attributes,
            timing: self.timing.clone(),
        }
    }

    pub fn with_timing(&self, timing: &'a FhirTiming, effective_from: &'a chrono::NaiveDateTime) -> Self {
        let updated_timing = self.timing.with_value(effective_from, timing);

        EntityOverTime {
            attributes: self.attributes.clone(),
            timing: updated_timing,
        }
    }

    pub fn between(&self, start: &'a chrono::NaiveDateTime, end: &'a chrono::NaiveDateTime) -> Vec<EntityOccurrence<'a>> {
        let mut occurrences: Vec<EntityOccurrence<'a>> = Vec::new();

        for timing_interval in self.timing.intervals.iter() {
            if timing_interval.interval.overlaps(&Interval { start, end: Some(end) }) {
                // TODO - For every version of the Timing that has overlapping validity with the query window
                // project occurrences using that Timing and collect them together. Use the attribute values
                // at the time of the occurrence.

                // let mut occurrence_attributes: HashMap<String, &'a String> = HashMap::new();
                // for (name, attr_over_time) in &self.attributes {
                //     if let Some(value) = attr_over_time.value_at(timing_interval.start) {
                //         occurrence_attributes.insert(name.clone(), value);
                //     }
                // }
                // occurrences.push(EntityOccurrence {
                //     attributes: occurrence_attributes,
                //     timing: timing_interval.value,
                // });
            }
        }

        // // For simplicity, we will just check the timing intervals and create occurrences
        // for timing_interval in &self.timing.intervals {
        //     if timing_interval.start >= start && (timing_interval.end.is_none() || timing_interval.end.unwrap() <= end) {
        //         let mut occurrence_attributes: HashMap<String, &'a String> = HashMap::new();
        //         for (name, attr_over_time) in &self.attributes {
        //             if let Some(value) = attr_over_time.value_at(timing_interval.start) {
        //                 occurrence_attributes.insert(name.clone(), value);
        //             }
        //         }
        //         occurrences.push(EntityOccurrence {
        //             attributes: occurrence_attributes,
        //             timing: timing_interval.value,
        //         });
        //     }
        // }

        occurrences
    }
}

#[cfg(test)]
mod tests {

}

//
// impl RecurrentVersionedEntity {
//     pub fn new() -> Self {
//         RecurrentVersionedEntity {
//             attributes: HashMap::new(),
//             timing: MultiAttributeInterval {
//                 intervals: Vec::new(),
//             },
//         }
//     }
//
//     pub fn new_with_initial_state(
//         attributes: HashMap<String, Box<dyn Any>>,
//         timing: FhirTiming,
//         effective_from: NaiveDateTime,
//     ) -> Self {
//         let mut attributes_intervals: HashMap<String, MultiAttributeInterval<Box<dyn Any>>> = HashMap::new();
//
//         for (name, value) in attributes {
//             let attribute_interval = AttributeInterval {
//                 start: effective_from,
//                 end: None,
//                 value,
//             };
//             let multi_intervals = MultiAttributeInterval {
//                 intervals: vec![attribute_interval],
//             };
//             attributes_intervals.insert(name, multi_intervals);
//         }
//
//         let mut timing_intervals: MultiAttributeInterval<FhirTiming> = MultiAttributeInterval {
//             intervals: Vec::new(),
//         };
//
//         timing_intervals.intervals.push(AttributeInterval {
//             start: effective_from,
//             end: None,
//             value: timing,
//         });
//
//         RecurrentVersionedEntity {
//             attributes: attributes_intervals,
//             timing: timing_intervals,
//         }
//     }
//
//     pub fn with_attributes(&self, attributes: HashMap<String, Box<dyn Any>>, effective_from: NaiveDateTime) -> Self {
//         let mut updated_attributes: HashMap<String, MultiAttributeInterval<Box<dyn Any>>> = self.attributes.clone();
//
//         for (name, value) in attributes {
//             let attribute_interval = AttributeInterval {
//                 start: effective_from,
//                 end: None,
//                 value,
//             };
//
//             let multi_intervals = if let Some(existing) = updated_attributes.get_mut(&name) {
//                 existing.intervals.push(attribute_interval);
//                 existing.clone()
//             } else {
//                 MultiAttributeInterval {
//                     intervals: vec![attribute_interval],
//                 }
//             };
//
//             updated_attributes.insert(name, multi_intervals);
//         }
//
//         RecurrentVersionedEntity {
//             attributes: updated_attributes,
//             timing: self.timing.clone(),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use std::any::Any;
//     use std::collections::HashMap;
//
//     use crate::timing::{FhirPeriod, FhirTimingRepeat};
//     // use super::{FhirTiming, RecurrentVersionedEntity};
//
//     // #[test]
//     // fn project_with_attribute_changes() {
//     //     let mut initial_attributes: HashMap<String, Box<dyn Any>> = HashMap::new();
//     //     initial_attributes.insert("id".to_owned(), Box::new("foobar".to_owned()));
//     //     let initial_timing = FhirTiming {
//     //         repeat: FhirTimingRepeat {
//     //             bounds: FhirPeriod {
//     //                 start: Some(chrono::NaiveDate::from_ymd_opt(2024, 12, 30).unwrap()),
//     //                 end: None,
//     //             },
//     //             frequency: 1,
//     //             period: 1,
//     //             period_unit: "d".to_string(),
//     //             day_of_week: vec![],
//     //             time_of_day: vec![],
//     //             when: vec!["MORN".to_owned()],
//     //         },
//     //     };
//     //     let effective_from = chrono::NaiveDateTime::from(chrono::NaiveDate::from_ymd_opt(2024, 12, 30).unwrap());
//     //     let versioned = RecurrentVersionedEntity::new_with_initial_state(
//     //         initial_attributes,
//     //         initial_timing,
//     //         effective_from,
//     //     );
//     //     let updated = versioned.with_attribute_interval()
//     //     // TODO - assert something
//     // }
// }
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
    use std::collections::HashMap;
    use chrono::NaiveDate;
    use crate::entity_over_time::EntityOverTime;
    use crate::timing::{FhirPeriod, FhirTiming, FhirTimingRepeat};

    #[test]
    fn task_schedule_over_time() {
        let dt1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let dt2 = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let dt3 = NaiveDate::from_ymd_opt(2024, 1, 3).unwrap().and_hms_opt(0, 0, 0).unwrap();

        let mut initial_attributes: HashMap<String, &String> = HashMap::new();
        let initial_attribute_val = "immutable_val".to_string();
        initial_attributes.insert("immutable".to_string(), &initial_attribute_val);
        let initial_mutable_attribute_val = "mutable_val".to_string();
        initial_attributes.insert("mutable".to_string(), &initial_mutable_attribute_val);

        let initial_timing = FhirTiming {
            repeat: FhirTimingRepeat {
                bounds: FhirPeriod {
                    start: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
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

        let updated_timing = FhirTiming {
            repeat: FhirTimingRepeat {
                bounds: FhirPeriod {
                    start: Some(NaiveDate::from_ymd_opt(2024, 1, 3).unwrap()),
                    end: None,
                },
                frequency: 1,
                period: 2,
                period_unit: "d".to_string(),
                day_of_week: vec![],
                time_of_day: vec![],
                when: vec![],
            },
        };

        let first_version = EntityOverTime::new_with_initial_state(initial_attributes, &initial_timing, &dt1);
        let updated_attribute_val = "mutable_val_v2".to_string();
        let second_version = first_version.with_attribute_value("mutable".to_string(), &updated_attribute_val, &dt2);
        let _ = second_version.with_timing(&updated_timing, &dt3);
    }
}

use chrono::NaiveDateTime;
use crate::interval::Interval;

#[derive(Clone)]
pub struct AttributeValueInterval<'a, T> where T: Clone {
    pub interval: Interval<'a>,
    pub value: &'a T,
}

#[derive(Clone)]
pub struct AttributeOverTime<'a, T> where T: Clone {
    pub intervals: Vec<AttributeValueInterval<'a, T>>,
}

impl<'a, T> AttributeOverTime<'a, T> where T: Clone {
    pub fn new() -> Self {
        AttributeOverTime {
            intervals: Vec::new(),
        }
    }

    pub fn new_with_value(start: &'a NaiveDateTime, value: &'a T) -> Self {
        AttributeOverTime {
            intervals: vec![AttributeValueInterval {
                interval: Interval { start, end: None },
                value,
            }],
        }
    }

    pub fn with_value(&self, start: &'a NaiveDateTime, value: &'a T) -> Self {
        let mut intervals = self.intervals.clone();
        if let Some(last) = intervals.last_mut() {
            if last.interval.end.is_none() {
                last.interval.end = Some(start);
            }
        }
        intervals.push(AttributeValueInterval {
            interval: Interval { start, end: None },
            value,
        });
        AttributeOverTime { intervals }
    }

    pub fn value_at(&self, at: &'a NaiveDateTime) -> Option<&T> {
        for value_interval in &self.intervals {
            if value_interval.interval.contains(at) {
                return Some(value_interval.value);
            }
        }
        None
    }
}

mod tests {
    use chrono::NaiveDate;
    use super::AttributeOverTime;

    #[test]
    fn test_attribute_interval() {
        let dt1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let dt2 = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let dt3 = NaiveDate::from_ymd_opt(2024, 1, 3).unwrap().and_hms_opt(0, 0, 0).unwrap();

        let attr1 = "Value1";
        let attr2 = "Value2";

        let multi_attr = AttributeOverTime::new_with_value(&dt1, &attr1);
        let updated_attr = multi_attr.with_value(&dt2, &attr2);

        assert_eq!(updated_attr.value_at(&dt1), Some(&"Value1"));
        assert_eq!(updated_attr.value_at(&dt2), Some(&"Value2"));
        assert_eq!(updated_attr.value_at(&dt3), Some(&"Value2"));
    }
}
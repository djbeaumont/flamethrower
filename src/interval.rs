use chrono::NaiveDateTime;

#[derive(Clone)]
pub struct Interval<'a> {
    pub start: &'a NaiveDateTime,
    pub end: Option<&'a NaiveDateTime>,
}

fn min<'a>(a: &'a NaiveDateTime, b: &'a NaiveDateTime) -> &'a NaiveDateTime {
    if a < b { a } else { b }
}

fn max<'a>(a: &'a NaiveDateTime, b: &'a NaiveDateTime) -> &'a NaiveDateTime {
    if a > b { a } else { b }
}

impl <'a> Interval<'a> {
    pub fn new(start: &'a NaiveDateTime, end: Option<&'a NaiveDateTime>) -> Self {
        Interval { start, end }
    }

    pub fn with_end(mut self, end: &'a NaiveDateTime) -> Self {
        self.end = Some(end);
        self
    }

    pub fn contains(&self, other: &'a NaiveDateTime) -> bool {
        self.start <= other && (self.end.is_none() || self.end.unwrap() > other)
    }

    pub fn overlaps(&self, other: &Interval<'a>) -> bool {
        self.start < other.end.unwrap_or(&NaiveDateTime::MAX) && other.start < self.end.unwrap_or(&NaiveDateTime::MAX)
    }

    pub fn intersection(&self, other: &Interval<'a>) -> Option<Interval<'a>> {
        if self.overlaps(other) {
            let start = max(self.start, other.start);
            let end = match (self.end, other.end) {
                (Some(e1), Some(e2)) => Some(min(e1, e2)),
                (Some(e1), None) => Some(e1),
                (None, Some(e2)) => Some(e2),
                (None, None) => None,
            };
            Some(Interval { start, end })
        } else {
            None
        }
    }
}

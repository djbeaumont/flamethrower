use chrono::prelude::*;

pub fn get_occurrences() -> Vec<DateTime<FixedOffset>> {
    let date = DateTime::parse_from_rfc3339("2021-06-21T21:46:18+01:00").unwrap();
    vec![date]
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let occurrences = super::get_occurrences();
        let first = occurrences.first().unwrap();
        assert_eq!(first.to_rfc3339(), "2021-06-21T21:46:18+01:00");
    }
}

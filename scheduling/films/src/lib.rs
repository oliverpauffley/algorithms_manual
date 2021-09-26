use std::cmp::Ordering;
use time::Date;

#[derive(Debug, Clone)]
pub struct Film {
    title: String,
    start_date: Date,
    end_date: Date,
}

impl Film {
    /// Create a new Film
    /// requires a start and end date with a title.
    ///
    /// # Panics
    ///
    /// panics if start date is after end date.
    pub fn new(title: String, start_date: time::Date, end_date: time::Date) -> Film {
        assert!(start_date < end_date);
        Film {
            title,
            start_date,
            end_date,
        }
    }

    /// Check if another Film overlaps with this film.
    /// returns true if any part of the other film is during the duration of this one.
    /// // TODO fix this
    pub fn overlaps(&self, other: &Film) -> bool {
        !(other.end_date <= self.start_date || other.start_date >= self.end_date)
    }

    pub fn title(self) -> String {
        self.title
    }
}

impl PartialOrd for Film {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.end_date.partial_cmp(&other.end_date)
    }
}

impl PartialEq for Film {
    fn eq(&self, other: &Self) -> bool {
        return self.start_date == other.start_date && self.end_date == other.end_date;
    }
}

impl Eq for Film {}

impl Ord for Film {
    fn cmp(&self, other: &Self) -> Ordering {
        self.end_date.cmp(&other.end_date)
    }
}

#[cfg(test)]
mod tests {
    use crate::Film;
    use time::Date;
    use time::Month;

    #[test]
    fn overlaps_returns_true_1() {
        let start_date_a = Date::from_calendar_date(2021, Month::January, 1).unwrap();
        let end_date_a = Date::from_calendar_date(2021, Month::January, 30).unwrap();
        let film_a = Film::new(String::from("Test Film A"), start_date_a, end_date_a);

        let start_date_b = Date::from_calendar_date(2021, Month::January, 15).unwrap();
        let end_date_b = Date::from_calendar_date(2021, Month::February, 10).unwrap();
        let film_b = Film::new(String::from("Test Film B"), start_date_b, end_date_b);

        assert!(
            film_a.overlaps(&film_b),
            "Should return true but returned false"
        );
    }

    #[test]
    fn overlaps_returns_true_2() {
        let start_date_a = Date::from_calendar_date(2021, Month::January, 16).unwrap();
        let end_date_a = Date::from_calendar_date(2021, Month::January, 30).unwrap();
        let film_a = Film::new(String::from("Test Film A"), start_date_a, end_date_a);

        let start_date_b = Date::from_calendar_date(2021, Month::January, 2).unwrap();
        let end_date_b = Date::from_calendar_date(2021, Month::January, 18).unwrap();
        let film_b = Film::new(String::from("Test Film B"), start_date_b, end_date_b);

        assert!(
            film_a.overlaps(&film_b),
            "Should return true but returned false"
        );
    }

    #[test]
    fn overlaps_returns_false() {
        let start_date_a = Date::from_calendar_date(2021, Month::January, 16).unwrap();
        let end_date_a = Date::from_calendar_date(2021, Month::January, 30).unwrap();
        let film_a = Film::new(String::from("Test Film A"), start_date_a, end_date_a);

        let start_date_b = Date::from_calendar_date(2021, Month::February, 2).unwrap();
        let end_date_b = Date::from_calendar_date(2021, Month::February, 18).unwrap();
        let film_b = Film::new(String::from("Test Film B"), start_date_b, end_date_b);

        assert!(
            !film_a.overlaps(&film_b),
            "Should return false but returned true"
        );
    }
}

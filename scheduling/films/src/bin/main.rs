use films::Film;

/// pick films returns the films you sign up to to maximize the
/// amount of time spent filming with the least possible downtime.
///
/// uses earliest completion algorithm to pick the best choices.
fn pick_films(films: &mut Vec<Film>) -> Vec<String> {
    let mut choices: Vec<String> = Vec::new();

    while !films.is_empty() {
        // find the first film to end
        let min_film = films.iter().min().unwrap().clone();
        println!("{:?}", &min_film);
        println!("{:?}", &films);
        // remove any overlapping choices
        films.retain(|film| !film.overlaps(&min_film));
        println!("{:?}", &films);

        choices.push(min_film.title());
    }
    choices
}

#[cfg(test)]
mod tests {
    use super::{pick_films, Film};
    use time::Date;
    use time::Month;

    #[test]
    fn returns_correct_film_choices() {
        let start_date_a = Date::from_calendar_date(2021, Month::January, 1).unwrap();
        let end_date_a = Date::from_calendar_date(2021, Month::January, 30).unwrap();
        let film_a = Film::new(String::from("Test Film A"), start_date_a, end_date_a);

        let start_date_b = Date::from_calendar_date(2021, Month::January, 15).unwrap();
        let end_date_b = Date::from_calendar_date(2021, Month::February, 10).unwrap();
        let film_b = Film::new(String::from("Test Film B"), start_date_b, end_date_b);

        let start_date_c = Date::from_calendar_date(2021, Month::January, 4).unwrap();
        let end_date_c = Date::from_calendar_date(2021, Month::January, 21).unwrap();
        let film_c = Film::new(String::from("Test Film C"), start_date_c, end_date_c);

        let start_date_d = Date::from_calendar_date(2021, Month::February, 4).unwrap();
        let end_date_d = Date::from_calendar_date(2021, Month::February, 16).unwrap();
        let film_d = Film::new(String::from("Test Film D"), start_date_d, end_date_d);

        let start_date_e = Date::from_calendar_date(2021, Month::February, 25).unwrap();
        let end_date_e = Date::from_calendar_date(2021, Month::March, 5).unwrap();
        let film_e = Film::new(String::from("Test Film E"), start_date_e, end_date_e);

        let mut films = vec![film_a, film_b, film_c, film_d, film_e];

        assert_eq!(
            pick_films(&mut films),
            vec!["Test Film C", "Test Film D", "Test Film E"]
        )
    }
}

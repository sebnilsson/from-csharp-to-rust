use super::GOOD_MOVIE_LIMIT;

pub fn run(_title: &str, year: u16) {
    let rating_avg = 7.8;

    let info = match rating_avg {
        x if x > GOOD_MOVIE_LIMIT => {
            format!("{} - You're in for a good movie", x)
        }
        x if x < GOOD_MOVIE_LIMIT && year <= 1995 => {
            format!("{} - Could be a classic...", x)
        }
        _ => String::from("... Do you want to look for another movie?"),
    };

    println!("[pattern_matching]");
    println!("info: {}", info);
}

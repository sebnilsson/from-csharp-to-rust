use super::GOOD_MOVIE_LIMIT;

pub fn run(title: &str, year: u16) {
    let mut info = String::from(title);
    let rating_avg = 7.8;

    if rating_avg >= GOOD_MOVIE_LIMIT {
        info += " - You're in for a good movie";
    } else if rating_avg < GOOD_MOVIE_LIMIT && year <= 1995 {
        info += "- Could be a classic...";
    } else {
        info += "... Do you want to look for another movie?";
    }

    let classic = year <= 1995 && !title.ends_with(" II");
    let classic_text =
        if classic { "Potential classic" } else { "We'll see..." };

    println!("[flow_logic]");
    println!("info: {}", info);
    println!("classic_text: {}", classic_text);
}

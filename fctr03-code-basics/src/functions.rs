use super::Movie;

pub fn run(movie: &Movie) {
    let sequel = is_sequel(&movie.title);

    println!("[functions]");
    println!("sequel: {}", sequel);
}

fn is_sequel(title: &str) -> bool {
    if title.is_empty() {
        return false;
    }
    let sequel_title = title.ends_with("II");

    sequel_title
}

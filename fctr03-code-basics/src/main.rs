#![allow(unused_assignments)]

const GOOD_MOVIE_LIMIT: f32 = 7.5;

mod collections;
mod functions;
mod logic_flow;
mod standard_output;
mod strings;
mod variables;

pub fn main() {
    println!("Hello, world!");

    let movie = Movie::new("Ghostbusters", 1984);
    println!("Movie: {}", movie.display_title());

    println!("-----");
    variables::run();
    println!("-----");
    strings::run(&movie.title);
    println!("-----");
    collections::run();
    println!("-----");
    logic_flow::run(&movie.title, movie.release_year);
    println!("-----");
    standard_output::run(&movie);
    println!("-----");
    functions::run(&movie);
}

#[derive(Debug)]
pub struct Movie {
    title: String,
    release_year: u16,
}

impl Movie {
    pub fn new(title: &str, release_year: u16) -> Self {
        Self { title: String::from(title), release_year }
    }

    pub fn display_title(&self) -> String {
        format!("{} ({})", self.title, self.release_year)
    }

    pub fn update_release_year(&mut self, year: u16) {
        self.release_year = year;
    }
}

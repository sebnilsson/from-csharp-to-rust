use super::Movie;

pub fn run(movie: &Movie) {
    let hello = "Hello";
    let world = "World";

    println!("[standard_output]");
    println!("{} {}!", hello, world);

    println!("Debug: {:?}", movie);
    println!("Debug pretty: {:#?}", movie);
}

use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let first_arg =
        if args.len() > 1 { args[1].clone() } else { String::default() };

    println!("[args]");
    println!("first_arg: {}", first_arg);
}

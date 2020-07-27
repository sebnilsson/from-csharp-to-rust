pub fn run(title: &str) {
    let mut value = String::default();
    for x in 65..68 {
        value.push(x as u8 as char);
    }

    // From str to String
    let title_string = String::from(title);
    // From String to str
    let title_str = title_string.as_str();

    println!("[strings]");
    println!("value: '{}'", value);
    println!("title_string: '{}', title_str: '{}'", title_string, title_str);
}

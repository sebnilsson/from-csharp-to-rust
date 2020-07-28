pub fn run() {
    println!("[null_values]");

    run_divide(2);
    run_divide(0);
}

fn run_divide(divisor: i32) {
    let quotient = divide(123, divisor);
    match quotient {
        Some(x) if x > 10 => println!("Big division result: {}", x),
        Some(x) => println!("Division result: {}", x),
        None => println!("Division failed")
    }
}

fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        return None;
    }
    Some(dividend / divisor)
}

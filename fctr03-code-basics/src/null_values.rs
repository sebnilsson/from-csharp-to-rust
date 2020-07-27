use super::Movie;

pub fn run(movie: &Movie) {
    println!("[null_values]");

    run_divide(2);
    run_divide(0);
}

fn run_divide(divisor: i32) {
    let quotient = divide(123, divisor);
    if quotient.is_some() {
        println!("Division result: {}", quotient.unwrap());
    } else {
        println!("Division failed");
    }
}

fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        return None;
    }
    Some(dividend / divisor)
}

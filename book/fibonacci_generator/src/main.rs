use std::env;

const FIB_0: u128 = 0;
const FIB_1: u128 = 1;

fn main() {
    let app_args = env::args();

    let n: u32 = match parse_args(app_args) {
        Ok(n) => n,
        Err(code) => {
            incorrect_usage(code);
            return;
        }
    };

    let fib: u128 = generate_fibonacci(n);
    println!("The fibonacci number {n} is {fib}");
}

fn parse_args(mut app_args: env::Args) -> Result<u32, i32> {
    app_args.next(); // skipping program name

    let n = match app_args.next() {
        Some(n) => n,
        None => return Err(-1),
    };

    match app_args.next() {
        Some(_s) => return Err(1),
        None => {}
    }

    match n.trim().parse::<u32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(0),
    }
}

fn generate_fibonacci(n: u32) -> u128 {
    match n {
        0 => FIB_0,
        1 => FIB_1,
        _ => {
            let mut prev = FIB_0;
            let mut curr = FIB_1;
            for _ in 1..n {
                (prev, curr) = (curr, prev.saturating_add(curr))
            }
            curr
        }
    }
}

fn incorrect_usage(error_code: i32) {
    // error_code 1 is for too many arguments
    // error_code -1 is for too few arguments
    // error_code 0 is for wrong argument type/value
    if error_code == 1 {
        println!("Error: Too many arguments.");
        println!("Usage: <program> <n>");
        println!("Example: cargo run -- 10");
    } else if error_code == -1 {
        println!("Error: Too few arguments.");
        println!("Usage: <program> <n>");
        println!("Example: cargo run -- 10");
    } else if error_code == 0 {
        println!("Error: Invalid input.");
        println!("Expected a non-negative integer.");
        println!("Usage: <program> <n>");
        println!("Example: cargo run -- 10");
    }
}

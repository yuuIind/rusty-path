use std::env;

fn main() {
    let app_args = env::args();
    let (value, unit) = match parse_args(app_args) {
        Ok((val, unit)) => (val, unit),
        Err(code) => {
            incorrect_usage(code);
            return;
        }
    };

    let new_value = convert(value, unit);
    let new_unit = if unit == 'C' { 'F' } else { 'C' };
    println!("{value}{unit} is {new_value}{new_unit}");
}

fn parse_args(mut app_args: env::Args) -> Result<(f64, char), i32> {
    app_args.next(); // skipping program name

    let value = match app_args.next() {
        Some(val) => val,
        None => return Err(-1),
    };
    let unit = match app_args.next(){
        Some(unit) => unit.trim().to_uppercase(),
        None => return Err(-1),
    };

    match app_args.next() {
        Some(_s) => return Err(1),
        None => {}
    }

    let value: f64 = match value.trim().parse() {
        Ok(val) => val,
        Err(_) => return Err(0),
    };

    match unit.as_str() {
        "F" => Ok((value, 'F')),
        "C" => Ok((value, 'C')),
        _ => Err(0)
    }
}

fn convert(value: f64, unit: char) -> f64 {
    match unit {
        'F' => convert_fah2cels(value),
        'C' => convert_cels2fah(value),
        _ => panic!("Unsupported unit: {}", unit),
    }
}

fn convert_cels2fah(value: f64) -> f64 {
    ((9.0 / 5.0) * value) + 32.0
}

fn convert_fah2cels(value: f64) -> f64 {
    (5.0 / 9.0) * (value - 32.0)
}

fn incorrect_usage(error_code: i32) {
    // error_code 1 is for too many arguments
    // error_code -1 is for too few arguments
    // error_code 0 is for wrong argument type/value
    if error_code == 1 {
        println!("Error: Too many arguments.");
        println!("Usage: ");
        println!("Example: cargo run -- 36.6 C");
    } else if error_code == -1 {
        println!("Error: Too few arguments.");
        println!("Usage: ");
        println!("Example: cargo run -- 36.6 C");
    } else if error_code == 0 {
        println!("Error: Invalid value or unit.");
        println!("Expected a number and a unit (F or C).");
        println!("Usage: ");
        println!("Example: cargo run -- 100 F");
    }
}

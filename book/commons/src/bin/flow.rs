fn main() {
    let condition: bool = false;

    if condition {
        println!("Condition is true: {condition}");
    } else {
        println!("Condition is false: {condition}");
    }

    let x = 1920;
    if x > 1923 {
        println!("x is greater than 1923: {x}");
    } else if x >= 1920 {
        println!("Cx is greater than or equal to 1920 but less than 1923: {x}");
    } else {
        println!("x is less than 1920: {x}");
    }
    
    let x = if condition { 19 } else { 23 };
    println!("The value of x is {x}");

    let mut x = 1;
    loop {
        if x > 5 {
            println!("Jail time is over, loop terminated");
            break;
        }
        println!("Turning around in the loop for {x} steps");
        x += 1;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter % 2 != 0 {
            continue
        }

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                println!("Breaking up with the nameless loop");
                break;
            }
            if count == 2 {
                println!("Breaking up with the counting_up loop");
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("COOLER LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
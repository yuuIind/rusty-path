use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the hidden number!");

    let secret = rand::thread_rng().gen_range(1..=100);

    println!("Please share your guess with me!");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your guess is supposed to be a number, you know… like 1, 92, or 3.");
                println!("Now, try again!");
                continue;
            }
        };

        println!("You guessed: {guess}");
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small! Consider a larger number."),
            Ordering::Greater => println!("Too big! An overestimation, as anticipated."),
            Ordering::Equal => {
                println!("You got it. Miracles do happen.");
                break;
            }
        }
    }
}

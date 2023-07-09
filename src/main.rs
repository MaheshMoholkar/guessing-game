use colored::Colorize;
use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..100);

    println!("Guess a number:");
    loop {
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read input");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        println!("You guessed: {}", number);

        match number.cmp(&random_number) {
            std::cmp::Ordering::Less => println!("{}", "Too small! Guess again:".red()),
            std::cmp::Ordering::Equal => {
                println!("{}", "You guessed correctly! Congratulations!".green());
                break;
            }
            std::cmp::Ordering::Greater => println!("{}", "Too large! Guess again:".red()),
        }
    }
}

use rand::Rng;
use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};

fn main() {
    // Display opening message
    println!("Guess the number!");
    println!("Choose your difficulty:");
    println!("\tEnter 1 for Easy (10 guesses)");
    println!("\tEnter 2 for Medium (6 guesses)");
    println!("\tEnter 3 for Hard (3 guesses)");

    let mut difficulty = String::new();

    // Read in user's difficulty choice
    stdin()
        .read_line(&mut difficulty)
        .expect("Failed to read user difficulty");

    let difficulty: u8 = match difficulty.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! Setting difficulty to Easy by default.");
            1
        }
    };

    // Set maximum allowed guesses based on difficulty choice
    let max_guesses: u8 = match difficulty {
        0 => return,
        1 => 10,
        2 => 6,
        3 => 3,
        4..=u8::MAX => return,
    };

    // Initialize random integer
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    // Set game loop
    for _n in 0..max_guesses {
        // Prompt user for their guess
        print!("\nPlease input your guess: ");
        // Stdout needs to be flushed to make print! display at the right time
        stdout().flush().expect("Failed to flush");

        // Initialize variable to hold user input
        let mut guess = String::new();

        // Read user guess into variable
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        // Convert user guess into an integer
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Guess again"),
            Ordering::Greater => println!("Too big! Guess again"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    // All guesses used up. Handle player loss
    println!("\nYou're out of guesses. The answer was {secret_number}");
    println!("Game over!");
}

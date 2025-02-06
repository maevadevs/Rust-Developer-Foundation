/// Guessing Game: Try to guess a randomly-generated number.

// Import libraries/modules
use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// The entry-point of the program.
fn main() {
    // Generate a random integer between 1 and 100, inclusive
    let secret_num: u32 = rand::rng().random_range(1..=100);

    loop {
        // Prompt the player to enter a guess
        println!("Take a guess, what number between 1 and 100?");

        // Process the user input
        let mut guess: String = String::new();

        // Check that the input is in the expected form
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Set explicit cast to u32
        // Handle user input errors
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That was not a valid number!");
                continue;
            }
        };

        // Confirm user's guess
        println!("You guessed: {guess}");

        // Handle when the number is too big or too small
        if guess < 1 || guess > 100 {
            println!("Your guess is outside of the accepted range.");
            continue;
        }

        // Indicate whether the guess is too low, too high, or correct
        // Compare guess vs secret_num
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // Print a congratulatory message and exit
                println!("You win!!!");
                break;
            }
        }
    }
}

// Check:               $ cargo check
// Build:               $ cargo build
// Build + Run:         $ cargo run
// Execute:             $ ./target/debug/guessing-game
// Build Release:       $ cargo build --release
// Build + Run Release: $ cargo run --release
// Execute Release:     $ ./target/release/guessing-game

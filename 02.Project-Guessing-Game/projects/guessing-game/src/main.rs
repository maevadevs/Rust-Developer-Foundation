// Import libraries/modules
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// The entry-point of the program.
fn main() {
    // Generate the secret number
    let secret_num = rand::thread_rng()
                          .gen_range(1..=100);

    loop {
        // Ask for user input
        println!("Take a guess, what number between 1 and 100?");

        // Process the user input
        let mut guess = String::new();

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
                continue
            }
        };

        // Confirm user's guess
        println!("You guessed: {guess}");

        // Compare guess vs secret_num
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!!");
                // Break the loop when number is guessed
                break;
            }
        }
    }
}

// Check:           $ cargo check
// Compile:         $ cargo build
// Build + Run:     $ cargo run
// Execute:         $ ./target/debug/guessing-game
// Build-Release:   $ cargo build --release
// Execute Release: $ ./target/release/guessing-game

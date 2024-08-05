# Project: Guessing Game

---

- [How It Works](#how-it-works)
- [Project Setup](#project-setup)
- [Processing A Guess](#processing-a-guess)
  - [`use std::io;`](#use-stdio)
  - [`fn main()`](#fn-main)
  - [`println!()`](#println)
  - [Storing Values With Variables](#storing-values-with-variables)
  - [Getting User Input](#getting-user-input)
- [Generating a Secret Number](#generating-a-secret-number)
  - [Using Crates](#using-crates)
  - [Ensuring Reproducible Builds with the `Cargo.lock` File](#ensuring-reproducible-builds-with-the-cargolock-file)
  - [Updating a Crate to Get a New Version](#updating-a-crate-to-get-a-new-version)
  - [Generating A Random Number](#generating-a-random-number)
- [Comparing Guess vs Secret Number](#comparing-guess-vs-secret-number)
- [Loop: Allowing Multiple Guesses](#loop-allowing-multiple-guesses)
  - [Quitting After Correct Guess](#quitting-after-correct-guess)
- [Handling Invalid Inputs](#handling-invalid-inputs)
- [Final Program Overall](#final-program-overall)

---

This is a *Project Chapter*

## How It Works

- Generate a random integer between 1 and 100
- Prompt the player to enter a guess
- After a guess is entered, indicate whether the guess is too low, too high, or correct
- If the guess is correct, print a congratulatory message and exit

## Project Setup

```sh
cargo new guessing-game
cd guessing-game
```

## Processing A Guess

1. Ask for user input
1. Process the user input
1. Check that the input is in the expected form
1. Confirm the user's guess

```rs
/// Guessing Game: Try to guess a randomly-generate number.

// Import libraries/modules
use std::io;

/// The entry-point of the program.
fn main() {
    // Prompt the player to enter a guess
    println!("Take a guess, what number between 1 and 100?");

    // Process the user input
    let mut guess: String = String::new();

    // Check that the input is in the expected form
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Confirm user's guess
    println!("You guessed: {guess}");
}
```

### `use std::io;`

- **For input/ouput operations, we use `std::io` package**
- **Rust Prelude**
  - By default, Rust brings *a set of items predefined in the `std` library* into the scope of every program
  - But this does not include all of `std`, only a portion called *Prelude*
  - [The list of *Rust Prelude* can be found here](https://doc.rust-lang.org/std/prelude)
  - **For things not in the *Prelude*, we have to manually import with `use` statements**

### `fn main()`

- `main()` is the entry to execution
  - No parameters
  - No return values
- **`fn` declares a new function**

### `println!()`

- ***Macro* that prints a string to the screen**
- **Supports *String Interpolation* with `{}`**
  - In the string, `{}` are considered *Placeholders* for variables
  - Variables inside `{}` will be evaluated
  - **However, it only supports *variables*, not *expressions***
  - To interpolate expressions, assign to variables first

```rs
let x = 5;
let y = 10;
let res = y + 2;

println!("x = {x} and y + 2 = {res}");

// The following will result in error: Trying to interpolate an expression
// println!("x = {x} and y + 2 = {y + 2}");
```

### Storing Values With Variables

```rs
let mut guess = String::new();
```

- `let`
  - Keyword to create a new variable
  - Allows to bind a value to the variable

```rs
// Immutable variable
let apple = 5;
```

- **In Rust, variables are *immutable* by default**
  - Once we give the variable a value, the value will not change
  - **It is almost like a constant, but a constant does not necessarily guarantee immutability**
    - `const` is evaluated during compile-time
    - Immutability is evaluated and assured even during runtime
  - *To make a variable mutable, add `mut` before the variable name when defining it*

```rs
// Mutable variable
let mut apple = 5;
```

- `String::new()`
  - A function that returns a new instance of a `String`
  - A *growable* string type provided by the standard library
  - Encoded in UTF-8
- **`::` is similar to `.`**

Token|Usage
:-:|:-
`.`|Used for *Value-member Access*
`::`|Used for *Namespace-member Access*

- `new()` is an *Associated Function* of `String`
  - **An *Associated Function* is a function that is implemented on a type**
  - `new()` is a common function on many types to create an *object* based on that type
  - `String::new()` creates an empty mutable string

Type|Value|Description
:-|:-:|:-
`str`|`""`|*Immutable* string
`String`|`String::new()`|*Mutable* string

### Getting User Input

```rs
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

- We use the `stdin()` function from the `std::io` module
  - Fullname: `std::io::stdin()`
  - Returns an instance of `std::io::Stdin`
  - Represents a handle to the standard input
- **Call `read_line()` method on the standard input handle**
  - Get input from the user
  - Store the result in `mut guess` variable
  - **Take whatever the user types into standard input and *append* that into a string `guess` without overwriting its contents**
    - The string argument needs to be mutable so the method can change/append the string’s content
  - **`&` indicates a *reference***
    - Access one piece of data without needing to copy that data into memory multiple times
    - Rust’s major advantages is how safe and easy it is to use references
    - **Like variables, references are immutable by default**
      - `&guess`: Immutable
      - `&mut guess`: Mutable
- **Handling Potential Failure with `Result`**
  - `read_line()` also returns a `Result<usize, Error>` value
  - `Result` is an *Enum*
    - Can be in one of multiple possible states
    - Each possible state is a *Variant*
  - `Result` types is to encode error-handling information
  - **`Result`’s variants are `Ok` and `Err`**
    - `Ok` - Success and contains the successfully generated value
    - `Err` - Failure and contains the failure information
  - **Values of `Result` type have methods**
    - `expect()` Method
    - If `Err`: Crash the program and display the message argument
    - If `Ok`: Return the value in `Ok` (Number of bytes in the user’s input)
  - **If we do not call `expect`, the program will compile, but we will get a warning**
    - This is part of Rust's *Error Handling*
    - The proper way to handle this is with *Error Handling*

## Generating a Secret Number

- Generate a secret number that the user will try to guess
  - The secret number should be different every time
  - Use a random number between 1 and 100
- *Random number generator is not part of Rust's Standard Library*
  - But there are some third-party crates that we can use
  - We can use the `rand` crate

### Using Crates

- **Crate**
  - Collection of Rust source code files
  - Similar to *Module* in Python
- **Binary Crate**
  - An executable crate
- **Library Crate**
  - Contains code that is intended to be used in other programs
  - Cannot be executed on its own
  - E.g. `rand` crate
- **Cargo manages Crates**
  - Modify the `Cargo.toml` file to include the `rand` crate as a dependency
  - `Cargo.toml` follows [*Semantic Versioning*](https://semver.org/)
  - The default SemVer specifier is `^`

```toml
# Filename: Cargo.toml
[dependencies]
rand = "^0.8.5"
```

- Everything that follows a `[header]` is part of that section that continues until another section starts
- **Run `cargo build` after changing `Cargo.toml`**
  - Cargo fetches the latest compatible versions of everything that the dependency needs from the *registry*
  - The *registry* is a copy of data from [Crates.io](https://crates.io/)
  - Collection of open-sourced Rust projects
- **After updating the registry, Cargo checks the `[dependencies]` section**
  - Downloads any crates listed that are not already downloaded
  - Also grabs all the dependencies of the dependencies (dependency-chain)
  - Rust compiles them and then compiles the project with the dependencies available
  - **Cargo only downloads or builds when needed and what changed**

### Ensuring Reproducible Builds with the `Cargo.lock` File

- Rebuild the same artifact every time you or anyone else builds your code
  - Only use the specified versions of the dependencies
  - **Rust creates the `Cargo.lock` file for the first time `cargo build` is run**
  - All the versions of the dependencies that fit the criteria
  - **If `Cargo.lock` file exists, Rust uses the versions specified rather than doing all the work of figuring out versions again**
- Allows to have a reproducible build automatically
- **`Cargo.lock` is often checked into source control**
  - Check-in for binaries/applications
  - Ignore for libraries
  - **When in doubt, check `Cargo.lock` in version control**
  - For details: [Why have `Cargo.lock` in version control?](https://doc.rust-lang.org/cargo/faq.html#why-have-cargolock-in-version-control)
- **To force to redownload all dependencies, delete `Cargo.lock` and re-run `cargo build`**
  - Or run `cargo update`

### Updating a Crate to Get a New Version

```sh
cargo update
```

- Ignores the `Cargo.lock` file
- Figure out all the latest versions that fit the specifications in `Cargo.toml`
- **To update to higher versions, modify the requirments in `Cargo.toml`**

### Generating A Random Number

```rs
// Import libraries/modules
use rand::Rng;
use std::io;

fn main() {
    // Generate a random integer between 1 and 100
    let secret_num: u32 = rand::thread_rng().gen_range(1..=100);

    // Ask for user input
    println!("Take a guess, what number?");

    // Process the user input
    let mut guess: String = String::new();

    // Check that the input is in the expected form
    io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read line");

    // Confirm user's guess
    println!("You guessed: {guess}");
}
```

- **`rand::Rng` is a *Trait***
  - Defines methods that random number generators implement
  - Must be in-scope to use those methods
- `rand::thread_rng()`
  - Gives the particular random number generator
  - Local to the current thread of execution
  - Seeded by the operating system
  - Call the `gen_range(start..=end)` method on the random number generator
    - Takes a *range* expression as an argument
    - Generates a random number in the range
    - `start..=end` is *inclusive on both end*

## Comparing Guess vs Secret Number

```rs
use std::cmp::Ordering;

fn main() {
    //...
    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Sorry, too small!"),
        Ordering::Greater => println!("Sorry, too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

- **`std::cmp::Ordering` is another enum type from the standard library**
  - Has the variants `Less`, `Greater`, and `Equal`
  - 3 outcomes that are possible when you compare two values
- **`cmp` method compares two values**
  - **Can be called on anything that can be compared**
  - Returns a variant of the `Ordering` enum
- **`match` expression**
  - Similar to `switch` in other languages
  - Based on the variant of `Ordering` returned from the call to `.cmp()`
  - `match` expression is made up of *arms*
    - **Arm** - A *pattern* to match against + the *code* that should be run if the value fits the pattern
    - Format: `Pattern => Code`
  - *Patterns* and the *match* construct are powerful Rust features
  - **NOTE: The `match` expression *breaks* on the first successful match**
- **`guess.cmp(&secret_num)` will return any of the possible enum values**
  - If `guess > secret_num`, it returns `Ordering::Greater`
  - If `guess < secret_num`, it returns `Ordering::Less`
  - If `guess == secret_num`, it returns `Ordering::Equal`
- ***NOTE: The code will not compile yet as-is***
  - `mismatched types` error
  - *Rust is a strongly-typed language with type inference*
  - It infered the type of `guess` as `String`
  - It infered the type of `secret_num` as `i32`
  - **Rust cannot compare a string and a number type**
    - We need to have an explicit cast

```rs
fn main() {
    let secret_num = rand::thread_rng().gen_range(1..=100);
    //...
    let mut guess = String::new();

    io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read line");

    // Set explicit cast
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");
    //...
}
```

- We (re)declare a variable `guess: u32`
  - **Rust allows to shadow the previous value of a variable with a new one**
  - *Shadowing* allows to reuse same variable name with different data type
  - This feature is often used when wanting to convert a value from one type to another type
  - **Note that the original data type is gone past this point**
- **`guess.trim().parse()`**
  - `trim()` eliminates any whitespace at the beginning and end of the string
    - User press `Enter` when submitting the guess
    - This creates a `\n` in the user input
    - We need to remove the extra `\n` before converting to `u32`
- **When comparing `guess` and `secret_num`, Rust will infer that `secret_num` should be a `u32` as well**
  - Comparison between 2 values of the same type
- **`parse()` only works on characters that can logically be converted into numbers**
  - It will fail if the string is not a number
  - Same as with `read_line()`, it returns a `Result` type
    - `parse()` returns an `Err Result` variant if it **cannot** create a number from the string
    - `parse()` returns an `Ok Result` variant if it **can** create a number from the string

## Loop: Allowing Multiple Guesses

- `loop` creates an infinite loop

```rs
//...
// Generate a random integer between 1 and 100
let secret_num: u32 = rand::thread_rng().gen_range(1..=100);

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
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // Confirm user's guess
    println!("You guessed: {guess}");

    // Compare guess vs secret_num
    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

- **But infinite loops are not good**
  - They will run forever
  - Unless the program crashes
  - **We want the game to quit when the correct number is guessed**

### Quitting After Correct Guess

- We can use `break` in the loop when the guess is correct
- Alter the `Ordering::Equal` condition

```rs
Ordering::Equal => {
    // Print a congratulatory message and exit
    println!("You win!!!");
    break;
}
```

- The loop is the last part of `main`
  - Exiting the loop also exits the program

## Handling Invalid Inputs

- Avoid crashing the program on bad input
- Ignore the bad input and allow user to `continue` to guess

```rs
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
```

- `expect` will crash the application
- **`match` allows to handle the error with the variant of `Result` enum from `parse()`**
  - If `Ok`, grab and use the value captured in `Ok`
  - If `Err`, we `continue` the loop to the next iteration
  - **`_` is a catch-all value: *Catch any error regardless of the contained value***

## Final Program Overall

```rs
/// Guessing Game: Try to guess a randomly-generate number.

// Import libraries/modules
use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// The entry-point of the program.
fn main() {
    // Generate a random integer between 1 and 100
    let secret_num: u32 = rand::thread_rng().gen_range(1..=100);

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
```

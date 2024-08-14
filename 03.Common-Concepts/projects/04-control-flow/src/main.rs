fn main() {
    println!();

    // Example of If-Else Expression
    // -----------------------------
    println!("Example of If-Else Expression:");
    println!("------------------------------");
    let number: i32 = 3;

    if number < 5 {
        println!(">> Condition was true");
    } else {
        println!(">> Condition was false");
    }
    println!();

    // Example of else if Expression
    // -----------------------------
    println!("Example of else if Expression:");
    println!("------------------------------");
    let number: i32 = 6;

    if number % 4 == 0 {
        println!(">> {number} is divisible by 4");
    } else if number % 3 == 0 {
        println!(">> {number} is divisible by 3");
    } else if number % 2 == 0 {
        println!(">> {number} is divisible by 2");
    } else {
        println!(">> {number} is not divisible by 4, 3, or 2");
    }
    println!();

    // Example of Using if With let
    // ----------------------------
    println!("Example of Using if With let:");
    println!("-----------------------------");
    let condition: bool = true;
    let number: i32 = if condition { 5i32 } else { 6i32 };

    println!(">> The value of number is: {number}");
    println!();
}

// Check:               $ cargo check
// Build:               $ cargo build
// Build + Run:         $ cargo run
// Execute:             $ ./target/debug/control-flow
// Build Release:       $ cargo build --release
// Build + Run Release: $ cargo run --release
// Execute Release:     $ ./target/release/control-flow

/// The main entry of the program.
fn main() {
    println!();

    // Example of Function
    // -------------------
    println!("Example of Function:");
    println!("--------------------");
    some_func();
    println!();

    // Example of Function With One Parameter
    // --------------------------------------
    println!("Example of Function With One Parameter:");
    println!("---------------------------------------");
    param_func(5);
    println!();

    // Example of Function With Multiple Parameters
    // --------------------------------------------
    println!("Example of Function With Multiple Parameters:");
    println!("---------------------------------------------");
    print_labeled_measurement(5, "cm");
    println!();

    // Example of Statements
    // ---------------------
    println!("Example of Statements:");
    println!("----------------------");
    let y: i32 = 6; // This is a statement
    println!("The value of y is {y}");
    println!();

    // Example of Expressions
    // ----------------------
    println!("Example of Expressions:");
    println!("-----------------------");
    let exp: i32 = {
        let y: i32 = 3;
        // Expressions do not end with semicolons
        // Else, it is considered a statement and does not return a value
        y + 1
    };

    println!("The value of exp is {exp}");
    println!();

    // Example of Function That Returns a Value
    // ----------------------------------------
    println!("Example of Function That Returns a Value:");
    println!("-----------------------------------------");
    let res: i32 = get_thousand();
    let res2: i32 = plus_one(res);
    println!("Value from calling get_thousand() is {res}");
    println!("Value from calling plus_one(res) is {res2}");
    println!();
}

/// Example of a function.
fn some_func() {
    println!("This is printing from some_func()");
}

/// Example of function with one parameter.
fn param_func(x: i32) {
    println!("The value of param is: {x}");
}

/// Example of function with multiple parameters.
fn print_labeled_measurement(value: i32, unit_label: &str) {
    println!("The measurement is: {value} {unit_label}");
}

/// Example of function that returns a value.
fn get_thousand() -> i32 {
    1000
}

/// Another Example of function that returns a value.
fn plus_one(x: i32) -> i32 {
    return x + 1;
}

// Check:               $ cargo check
// Build:               $ cargo build
// Build + Run:         $ cargo run
// Execute:             $ ./target/debug/functions
// Build Release:       $ cargo build --release
// Build + Run Release: $ cargo run --release
// Execute Release:     $ ./target/release/functions

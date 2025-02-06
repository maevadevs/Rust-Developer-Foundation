// Example of Global Constants
// ---------------------------
// Constants are always immutable
// They are basically always read-only
// Constant values must be determined at compile-time
// Constants can be declared in any scope, including the global scope
const SECONDS_IN_HOUR: u32 = 60 * 60;
const PI: f64 = 3.14159265359;

/// The main entry of the program.
fn main() {
    println!();

    // Immutable Variable
    // ------------------
    // Variable is immutable by default
    let my_int: i32 = 55;

    println!("Immutable Variable:");
    println!("-------------------");
    println!("immutable my_int = {my_int}");
    println!();

    // Reassigning to an immutable variable is a compile-time error
    // my_int = 6;
    // => error[E0384]: cannot assign twice to immutable variable `my_int`

    // Mutable Variable
    // ----------------
    // Variable is immutable by default
    // But adding keyword `mut` makes it mutable
    let mut mut_int: i32 = 78;

    println!("Mutable Variable:");
    println!("-----------------");
    println!("mutable mut_int = {mut_int}");

    // This line will not generate an error
    mut_int = 1024;

    println!("Now, mutable mut_int = {mut_int}");
    println!();

    // Examples of Constants
    // ---------------------
    // Constants are always immutable
    // They are basically always read-only
    // Constant values must be determined at compile-time
    println!("Examples of Constants:");
    println!("----------------------");
    println!("SECONDS_IN_HOUR = {SECONDS_IN_HOUR}");
    println!("PI = {PI}");
    println!();

    // Variable Shadowing and Scope
    // ----------------------------
    // Variable Shadowing is not the same as `mut`
    // Variable Shadowing redeclares the variable with `let`
    // The variable itself does not mutate:
    // We are creating a new variable each time
    let my_int: i32 = my_int + 6;

    println!("Examples of Variable Shadowing and Scopes:");
    println!("------------------------------------------");
    println!("Local-Scope: my_int = {my_int}");
    {
        // In a different scope, this also shadows the same my_int above
        // In this block, the "outside" variable temporarily halt from being seen
        let my_int: i32 = my_int * 2;
        println!("Inside-Scope: my_int = {my_int}");
    }
    // After the scope ends, the shadowing also ends
    // This one is back to the previously-shadowed "outside" my_int
    println!("Local-Scope: my_int = {my_int}");
    println!();

    // Example of Changing Variable Type While Shadowing:
    // --------------------------------------------------
    println!("Example of Changing Variable Type While Shadowing:");
    println!("--------------------------------------------------");
    let spaces: &str = "   x    ";          // String type: Non-mutable
    println!("Before: spaces = {spaces}");
    let spaces: usize = spaces.len();       // Number type: Non-mutable
    println!("After: spaces = {spaces}");

    println!();
}

// Check:               $ cargo check
// Build:               $ cargo build
// Build + Run:         $ cargo run
// Execute:             $ ./target/debug/variables
// Build Release:       $ cargo build --release
// Build + Run Release: $ cargo run --release
// Execute Release:     $ ./target/release/variables

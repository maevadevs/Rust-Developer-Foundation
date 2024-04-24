// Example of Global Constants
// ---------------------------
const HOUR_IN_SECONDS: u32 = 60 * 60;
const PI: f64 = 3.14159265359;

/// The main entry of the program.
fn main() {
    println!();

    // Immutable Variable
    // ------------------
    // Variable is immutable by default
    let x = 55;

    println!("Immutable Variable:");
    println!("immutable x = {x}");
    println!();

    // Reassigning to an immutable variable is a compile-time error
    // This line will generate an error
    // x = 6;
    // error[E0384]: cannot assign twice to immutable variable `x`

    // Mutable Variable
    // ----------------
    // Variable is immutable by default
    // But adding keyword `mut` makes it mutable
    let mut y = 78;

    println!("Mutable Variable:");
    println!("mutable y = {y}");

    // This line will not generate an error
    y = 1024;

    println!("Now, mutable y = {y}");
    println!();

    // Examples of Constants
    // ---------------------
    // Constants are always immutable
    // They are basically always read-only
    println!("Examples of Constants:");
    println!("HOUR_IN_SECONDS = {HOUR_IN_SECONDS}");
    println!("PI = {PI}");
    println!();

    // Variable Shadowing and Scope
    // ----------------------------
    // Variable Shadowing is not the same as `mut`
    // Variable Shadowing redeclares the variable with `let`
    // The variable itself does not mutate:
    // We are creating a new variable each time
    let x = x + 6;

    println!("Examples of Variable Shadowing and Scopes:");
    println!("Local-Scope: x = {x}");
    {
        // In a different scope, this also shadows the same x
        let x = x * 2;
        println!("Inside-Scope: x = {x}");
    }
    // After the scope ends, the shadowing also ends
    // This one is back to the previous shadow
    println!("Local-Scope: x = {x}");
    println!();
}

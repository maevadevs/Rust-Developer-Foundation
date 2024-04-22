// Example of global constants
const HOUR_IN_SECONDS: u32 = 60 * 60;
const PI: f64 = 3.14159265359;

fn main() {
    // Variables and Immutability
    // --------------------------
    // Variable is immutable by default
    let x = 55;
    println!("The value of x is: {x}");
    println!();

    // Reassigning to an immutable variable is a compile-time error
    // This line will generate an error
    // x = 6;
    // error[E0384]: cannot assign twice to immutable variable `x`

    // Variable is immutable by default
    // But adding keyword `mut` makes it mutable
    let mut y = 78;
    println!("The value of y is: {y}");

    // This line will not generate an error
    y = 1024;
    println!("Now, the value of y is: {y}");
    println!();

    // Constants are always immutable
    // They are basically always read-only
    println!("1 Hour = {HOUR_IN_SECONDS} seconds");
    println!("PI = {PI}");
    println!();

    // Variable Shadowing is not the same as `mut`
    // Variable Shadowing redeclares the variable with `let`
    // The variable itself does not mutate:
    // We are creating a new variable each time
    let x = x + 6;
    println!("Local-Scope: The value of x is now: {x}");
    {
        // In a different scope, this also shadows the same x
        let x = x * 2;
        println!("Inside-Scope: The value of x in the inner scope is: {x}");
    }
    // After the scope ends, the shadowing also ends
    // This one is back to the previous shadow
    println!("Local-Scope: The value of x is back to: {x}");
    println!();

    // Examples of Signed Integers
    // ---------------------------
    let byte: i8 = -128;
    let short: i16 = -32_768;
    let int: i32 = -2_147_483_648;
    let long: i64 = -9_223_372_036_854_775_808;
    let llong: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;

    println!("Example of Signed Integers:");
    println!("i8 = {byte}");
    println!("i16 = {short}");
    println!("i32 = {int}");
    println!("i64 = {long}");
    println!("i128 = {llong}");
    println!();

    // Examples of Unsigned Integers
    let ubyte: u8 = 255;
    let ushort: u16 = 65_535;
    let uint: u32 = 4_294_967_295;
    let ulong: u64 = 18_446_744_073_709_551_615;
    let ullong: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;

    println!("Example of Unsigned Integers:");
    println!("u8 = {ubyte}");
    println!("u16 = {ushort}");
    println!("u32 = {uint}");
    println!("u64 = {ulong}");
    println!("u128 = {ullong}");
    println!();

    // Examples of Floating-Points
    // ---------------------------
    let max_float32: f32 = f32::MAX;
    let min_float32: f32 = f32::MIN;
    let max_float64: f64 = f64::MAX;
    let min_float64: f64 = f64::MIN;
    let max_float32_repr: String = format!("{:+e}", max_float32);
    let min_float32_repr: String = format!("{:+e}", min_float32);
    let max_float64_repr: String = format!("{:+e}", max_float64);
    let min_float64_repr: String = format!("{:+e}", min_float64);

    println!("Example of Floating-Points:");
    println!("{min_float32_repr} <= f32 <= {max_float32_repr}");
    println!("{min_float64_repr} <= f64 <= {max_float64_repr}");
    println!();

    // Examples of Numeric Operations
    // ------------------------------
    // Addition
    let sum = 5 + 10;
    // Subtraction
    let difference = 95.5 - 4.3;
    // Multiplication
    let product = 4 * 30;
    // Division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // Remainder
    let remainder_i32 = 43 % 5;
    let remainder_f64 = 43.5 % 5.6;

    println!("Example of Numeric Operations:");
    println!("5 + 10 = {sum}");
    println!("95.5 - 4.3 = {difference}");
    println!("4 * 30 = {product}");
    println!("56.7 / 32.2 = {quotient}");
    println!("-5 / 3 = {truncated}");
    println!("43 % 5 = {remainder_i32}");
    println!("43.5 % 5.6 = {remainder_f64}");
    println!();

    // Examples of Booleans
    // --------------------
    let answer: bool = true;
    let reply: bool = false;

    println!("Example of Booleans:");
    println!("answer = {answer}");
    println!("reply = {reply}");
    println!();

    // Examples of Characters
    // ----------------------
    let small = 'z';
    let special: char = '\u{20AC}'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("Example of Characters:");
    println!("small = {small}");
    println!("special = {special}");
    println!("heart_eyed_cat = {heart_eyed_cat}");
    println!();

    // Example of Tuples
    // -----------------
    let tup: (i32, f64, i8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let equal_x = x == tup.0;
    let equal_y = y == tup.1;
    let equal_z = z == tup.2;

    println!("Example of Tuples:");
    println!("tup = {tup:?}");
    println!("x = {x}");
    println!("y = {y}");
    println!("z = {z}");
    println!("x == tup.0 ? {equal_x}");
    println!("x == tup.1 ? {equal_y}");
    println!("x == tup.2 ? {equal_z}");
    println!();

    // Example of an Array
    // -------------------
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    const MONTHS: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    // Example of an Array with same element repeated
    let arr_10 = [5; 10];

    println!("Example of Array:");
    println!("arr = {arr:?}");
    println!("MONTHS = {MONTHS:#?}");
    println!("arr_10 = {arr_10:?}");
    println!();

    // Example of Accessing Array Elements
    // -----------------------------------
    let some_nums = [1, 2, 3, 4, 5];
    let first = some_nums[0];
    let second = some_nums[1];

    println!("Example of Accessing Array Elements:");
    println!("some_nums = {some_nums:?}");
    println!("first = {first}");
    println!("second = {second}");
    println!()

    
}

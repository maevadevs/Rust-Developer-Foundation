fn main() {
    println!();

    // Example of Creating a String
    // ----------------------------
    // NOTE: String can be mutated
    println!("Example of Creating a String:");
    println!("-----------------------------");

    let mut st: String = String::from("Hello World!");
    // push_str() appends a literal to a String
    st.push_str(" ");
    st.push_str("Hello everyone!");
    println!("st = {st}");

    // Example of immutable string (literal string)
    let greet: &str = "Hi there!";
    println!("greet = {greet}");
    println!();

    // Example of Variables and Data Interacting with *Move*
    // -----------------------------------------------------
    println!("Example of Variables and Data Interacting with *Move*:");
    println!("------------------------------------------------------");

    // Bind the value "hello" to s1
    let s1: String = String::from("hello");
    // This does not make a separate copy of s1
    // Only copy the pointer, length, and capacity
    let s2: String = s1;

    // println!("s1 = {s1}!"); // error[E0382]: borrow of moved value: `s1`
    println!("s2 = {s2}");
    println!();

    // Example of Variables and Data Interacting with *Clone*
    // ------------------------------------------------------
    println!("Example of Variables and Data Interacting with *Clone*:");
    println!("-------------------------------------------------------");

    let s1: String = String::from("hello");
    let s2: String = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
    println!();

    // Example of Ownership With Functions
    // -----------------------------------
    println!("Example of Ownership With Functions:");
    println!("------------------------------------");

    let st: String = String::from("hello");  // st comes into scope

    takes_ownership(st);    // st's value moves into the function
    // println!("st = {st}");         // so st is no longer valid here

    let x: i32 = 5;                   // x comes into scope

    makes_copy(x);         // x would move into the function,
    println!("x = {x}");              // but i32 is Copy, so it's okay to still use x afterward
    println!();

    // Example of Return Values and Ownership
    // --------------------------------------
    println!("Example of Return Values and Ownership:");
    println!("---------------------------------------");

    let s1: String = gives_ownership(); // gives_ownership() moves its return value into s1
    let s2: String = String::from("hello"); // s2 comes into scope
    let s3: String = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back()
                                                        // which also moves its return value into s3

    println!("s1 = {s1}");
    // println!("s2 = {s2}"); // s2 has moved
    println!("s3 = {s3}");
    println!();

    println!("Example of Borrowing With Reference:");
    println!("------------------------------------");

    let s1: String = String::from("hello");
    let len: usize = calculate_length(&s1); // s1 is borrowed by the function via reference
    println!("The length of '{s1}' is {len}.");
    println!();

    println!("Example of Mutable Reference");
    println!("----------------------------");

    // The target variable needs to be mutable
    let mut s2: String = String::from("hello");
    println!("Before: s2 = {s2}");
    change(&mut s2); // Attempting to change a mutable borrowed value
    println!("After: s2 = {s2}");
    println!();

    {
        let mut st: String = String::from("hello");

        let mut_r1: &mut String = &mut st; // 1st Mutable Reference
        // let mut_r2: &mut String = &mut st; // 2nd Mutable Reference: Error: Cannot borrow a 2nd time

        println!("{mut_r1}");
        println!("{mut_r1}"); // Last time the 1st Mutable Reference is used within this block. End of scope for mut_r1

        let mut_r2: &mut String = &mut st; // 2nd Mutable Reference: We can now borrow as 1st mutable reference scope has ended
        println!("{mut_r2}");
    }
}
// Here, s3 goes out of scope and is dropped
// s2 was moved into takes_and_gives_back(), so nothing happens
// s1 goes out of scope and is dropped
// Here, x goes out of scope, then st
// But because st's value was moved, nothing special happens

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("some_string = {some_string}");
}
// Here, some_string goes out of scope and `drop` is called
// The backing memory is freed

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("some_integer = {some_integer}");
}
// Here, some_integer goes out of scope
// Nothing special happens

/// gives_ownership() will move its return value into the function that calls it.
fn gives_ownership() -> String {
    let some_string: String = String::from("yours"); // some_string comes into scope
    return some_string; // some_string is returned and moves out to the calling function
}

/// takes_and_gives_back() takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    return a_string;  // a_string is returned and moves out to the calling function
}

/// A function that borrows a reference
fn calculate_length(st: &String) -> usize {
    return st.len();
}
// Here, st goes out of scope.
// But because it does not have ownership of what it refers to, it is not dropped.

/// A borrowing function: Parameter is mutable.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Check:               $ cargo check
// Build:               $ cargo build
// Build + Run:         $ cargo run
// Execute:             $ ./target/debug/ownership
// Build Release:       $ cargo build --release
// Build + Run Release: $ cargo run --release
// Execute Release:     $ ./target/release/ownership

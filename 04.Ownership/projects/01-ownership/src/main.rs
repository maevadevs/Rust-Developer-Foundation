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


}

// Check:               $ cargo check
// Build:               $ cargo build
// Build + Run:         $ cargo run
// Execute:             $ ./target/debug/ownership
// Build Release:       $ cargo build --release
// Build + Run Release: $ cargo run --release
// Execute Release:     $ ./target/release/ownership

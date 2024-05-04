fn main() {
    println!();

    // Example of Creating a String
    // ----------------------------
    // NOTE: String can be mutated
    println!("Example of Creating a String:");

    let mut st = String::from("Hello World!");
    // push_str() appends a literal to a String
    st.push_str(" ");
    st.push_str("Hello everyone!");
    println!("{}", st);
    println!();
}

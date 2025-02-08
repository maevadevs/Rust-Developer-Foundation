fn main() {
    println!();

    // Example of Solving Problem Without Using Slice
    // ----------------------------------------------
    // NOTE: String can be mutated
    println!("Example of Solving Problem Without Using Slice:");
    println!("-----------------------------------------------");

    let hello: usize = first_word(&String::from("Hello World!"));
    println!("Hello World! -> {hello}"); // => 5

    let abraca: usize = first_word(&String::from("Abracadabra!"));
    println!("Abracadabra! -> {abraca}"); // => 12
    println!();

    // Example of Solving Problem Using Slice
    // --------------------------------------
    // NOTE: String can be mutated
    println!("Example of Solving Problem Using Slice:");
    println!("---------------------------------------");

    let st: String = String::from("Hello again world!");

    let hello: &str = &st[0..5];
    let again: &str = &st[6..11];
    let world: &str = &st[12..18];

    println!("Sentence: {st}");
    println!("1st word: {hello}");
    println!("2nd word: {again}");
    println!("3rd word: {world}");
    println!();

    // Example of Slicing at Non-UTF-8 Character Boundary
    // --------------------------------------------------
    println!("Example of Slicing at Non-UTF-8 Character Boundary:");
    println!("---------------------------------------------------");

    let st: String = String::from("Hello ❤️!");
    let st_len: usize = st.len();
    println!("{st} => len = {st_len}"); // 13

    // // Attempting to index in the middle of the ❤️ character
    // let bad_utf_index: &str = &st[7..10];
    // println!("{bad_utf_index}");
    // // Error: byte index 7 is not a char boundary; it is inside '❤' (bytes 6..9) of `Hello ❤️!`
    println!();

    // Example of resolving Problem Using Slice
    // ----------------------------------------
    // NOTE: String can be mutated
    println!("Example of resolving Problem Using Slice:");
    println!("-----------------------------------------");

    let original: String = String::from("Hello World!");
    let hello: &str = first_word_slice(&original);
    println!("Hello World! -> {hello}");

    let original: String = String::from("Abracadabra");
    let abraca: &str = first_word_slice(&original);
    println!("Abracadabra! -> {abraca}");
    println!();

    // Example of Using string slice vs Reference to a String
    // ------------------------------------------------------
    println!("Example of Using string slice vs Reference to a String:");
    println!("-------------------------------------------------------");
    let my_string: String = String::from("hello world");
    println!("my_string: String = {my_string}");

    // `first_word_slice` works on slices of `String`s, whether partial or whole
    let word1: &str = first_word_slice(&my_string[0..6]);
    let word2: &str = first_word_slice(&my_string[..]);
    println!("\tPartial Slice = {word1}");
    println!("\tWhole Slice = {word2}");

    // `first_word_slice` also works on references to `String`s,
    // which are equivalent to whole slices of `String`s
    let word3: &str = first_word_slice(&my_string);
    println!("\tReference to String = {word3}");
    println!();

    let my_string_literal: &str = "hello world again";
    println!("my_string_literal: &str = {my_string_literal}");

    // `first_word_slice` works on slices of string literals, whether partial or whole
    let word4: &str = first_word_slice(&my_string_literal[0..6]);
    let word5: &str = first_word_slice(&my_string_literal[..]);
    println!("\tPartial Slice = {word4}");
    println!("\tWhole Slice = {word5}");

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word6: &str = first_word_slice(my_string_literal);
    println!("\tJust String Literal = {word6}");
    println!();

    // Example of Slice of Array
    // -------------------------
    println!("Example of Slice of Array");
    println!("-------------------------");

    // Original array
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    // Array Slice
    let arr_slice: &[i32] = &arr[1..3];

    let is_equal: bool = arr_slice == &[20, 30];
    print!("arr = ");
    for el in arr {
        print!("{el} ");
    }
    println!();
    print!("arr_slice = ");
    for el in arr_slice {
        print!("{el} ");
    }
    println!();
    println!("arr_slice == &[20, 30] ? => {is_equal}");
    println!();
}


/// Get the ending index of the first word of a string,
/// or the end of the string if it is a single word.
fn first_word(st: &String) -> usize {
    // Convert the string to bytes
    let bytes: &[u8] = st.as_bytes();

    // Loop through the bytes
    for (i, &item) in bytes.iter().enumerate() {
        // If ' ' found, return the index
        if item == b' ' {
            return i;
        }
    }

    // If no ' ' found, index is at the end of the string
    st.len()
}

/// Get the first word of a string,
/// or the original string if it is a single word.
/// NOTE: Using parameter &str is more flexible than using &String
fn first_word_slice(st: &str) -> &str {
    // Convert the string to bytes
    let bytes: &[u8] = st.as_bytes();

    // Loop through the bytes
    for (i, &item) in bytes.iter().enumerate() {
        // If ' ' found, return the string slice
        if item == b' ' {
            return &st[0..i];
        }
    }

    // If no ' ' found, return the entire string
    &st[..]
}

// Check:               $ cargo check
// Build:               $ cargo build
// Build + Run:         $ cargo run
// Execute:             $ ./target/debug/slice
// Build Release:       $ cargo build --release
// Build + Run Release: $ cargo run --release
// Execute Release:     $ ./target/release/slice

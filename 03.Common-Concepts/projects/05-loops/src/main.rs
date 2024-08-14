fn main() {
    println!();

    // Example of Infinite Loops Using `loop`
    // --------------------------------------
    // println!("Example of Infinite Loops Using `loop`:");
    // println!("---------------------------------------");
    // loop {
    //     println!("run again!");
    // }
    // println!();

    // Example of Controlled Loops Using `loop` and `break`
    // ----------------------------------------------------
    println!("Example of Controlled Loops Using `loop` and `break`:");
    println!("-----------------------------------------------------");
    let mut i: i32 = 0;
    loop {
        if i == 10 {
            break;
        }
        println!("run again!");
        i += 1;
    }
    println!();

    // Example of Returning Values From Loop
    // -------------------------------------
    println!("Example of Returning Values From Loop");
    println!("-------------------------------------");
    let mut counter: i32 = 0;

    // Capture the returned value from the loop
    let result: i32 = loop {
        if counter == 10 {
            // Return the value from the loop
            break counter * 2;
        }
        counter += 1;
    };

    println!("The result from the loop is {result}");
    println!();

    // Example of Using Loop Label
    // ---------------------------
    println!("Example of Using Loop Label");
    println!("---------------------------");
    let mut count: i32 = 0;

    // Loop label for outer loop
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining: i32 = 10;

        // Inner loop
        loop {
            println!("\tremaining = {remaining}");
            if remaining == 5 {
                // Break from the inner loop
                break;
            }
            if count == 2 {
                // Break from the outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    println!();

    // Example of while Loop
    // ---------------------
    println!("Example of while Loop");
    println!("---------------------");
    let mut number: i32 = 5;

    while number != 0 {
        print!("{number}... ");
        number -= 1;
    }

    println!("LIFTOFF!!!");
    println!();

    // Example of Using while Loop Over Array
    // --------------------------------------
    println!("Example of Using while Loop Over Array");
    println!("--------------------------------------");
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;

    while index < 5 {
        println!("The value is: {}", arr[index]);
        index += 1;
    }
    println!();

    // Example of Using for Loop Over Array
    // ------------------------------------
    println!("Example of Using for Loop Over Array");
    println!("------------------------------------");
    let arr: [i32; 5] = [10, 20, 30, 40, 50];

    for el in arr {
        println!("The value is: {el}");
    }
    println!();

    // Using Range With for-Loops
    // --------------------------
    println!("Using Range With for-Loops");
    println!("--------------------------");
    for num in (1..6).rev() {
        print!("{num}... ");
    }
    println!("LIFTOFF!!!");
    println!();
}

// Check:               $ cargo check
// Build:               $ cargo build
// Build + Run:         $ cargo run
// Execute:             $ ./target/debug/loops
// Build Release:       $ cargo build --release
// Build + Run Release: $ cargo run --release
// Execute Release:     $ ./target/release/loops

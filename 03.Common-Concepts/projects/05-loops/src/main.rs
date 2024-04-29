fn main() {
    println!();

    // Example of Infinite Loops Using `loop`
    // --------------------------------------
    // println!("Example of Infinite Loops Using `loop`:");
    // loop {
    //     println!("again!");
    // }
    // println!();

    // Example of Returning Values From Loop
    // -------------------------------------
    let mut counter = 0;

    // Capture the value from the loop
    let result = loop {
        counter += 1;
        if counter == 10 {
            // Return the value from the loop
            break counter * 2;
        }
    };

    println!("The result from the loop is {result}");
    println!();

    // Example of Using Loop Label
    // ---------------------------
    let mut count = 0;

    // Loop label for outer loop
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        // Inner loop
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
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

    // Example of while loop
    // ---------------------
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
    println!();

    // Example of for loop Over Array
    // ------------------------------
    let a = [10, 20, 30, 40, 50];
    for el in a {
        println!("The value is: {el}");
    }
    println!();

    // Using Range With for-loops
    // --------------------------
    for num in (1..4).rev() {
        println!("{num}!");
    }
    println!("LIFTOFF!!!");
    println!();
}

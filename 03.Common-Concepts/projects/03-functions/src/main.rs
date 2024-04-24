/// The main entry of the program.
fn main() {
    println!();

    // Example of Function
    // -------------------
    println!("Example of Function:");
    some_func();
    println!();

    // Example of Function With One Parameter
    // --------------------------------------
    println!("Example of Function With Parameter:");
    param_func(5);
    println!();

    // Example of Function With Multiple Parameters
    // --------------------------------------------
    println!("Example of Function With Multiple Parameters:");
    print_labeled_measurement(5, "cm");
    println!();

    // Example of Statements
    // ---------------------
    println!("Example of Statements:");
    let y = 6;
    println!("The value of y is {y}");
    println!();

    // Example of Expressions
    // ----------------------
    println!("Example of Expressions:");
    let exp = {
        let y = 3;
        y + 1
    };

    println!("The value of exp is {exp}");
    println!();

    // Example of Function That Return a Value
    // ---------------------------------------
    println!("Example of Function That Return a Value:");
    let res = get_thousand();
    let res2 = plus_one(res);
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

/// Example of function that return a value.
fn get_thousand() -> i32 {
    1000
}

/// Another Example of function that return a value.
fn plus_one(x: i32) -> i32 {
    x + 1
}

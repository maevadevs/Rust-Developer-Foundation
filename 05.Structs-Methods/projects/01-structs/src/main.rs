fn main() {
    println!();
    println!("Example of Instantiating a Struct");
    println!("---------------------------------");

    // Instantiating a Struct
    let mut user1: User = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1,
    };

    // Modifying a field's value
    // NOTE: The Struct needs to be mutable
    user1.sign_in_count += 1;

    // Accessing Struct fields
    // We cannot use `.` directly in string interpolations
    // Use *borrow* so the ownership does not *move*
    let user_name: &String = &user1.username;
    let user_email: &String = &user1.email;
    let user_signin_count: &u64 = &user1.sign_in_count;

    println!("Your username is {user_name}.");
    println!("You email is {user_email}.");
    if user1.active {
        println!("Your account is currently active.")
    } else {
        println!("Your account is currently inactive.")
    }
    println!("Your sign-in count: {user_signin_count}.");
    println!();

    println!("Example of Returning a Struct From a Function");
    println!("---------------------------------------------");

    let user1: User = build_user(String::from("dummy@email.com"), String::from("anonymous123"));
    let user_name: &String = &user1.username;
    let user_email: &String = &user1.email;

    println!("Your username is {user_name}.");
    println!("You email is {user_email}.");
    println!();

    println!("Example of Struct Update Syntax");
    println!("-------------------------------");

    let user2: User = User {
        // Set the email as different value
        email: String::from("another@example.com"),
        // And everything else is the same as user1
        ..user1
    };

    // Use *borrow* so the ownership does not *move*
    let user_name: &String = &user2.username;
    let user_email: &String = &user2.email;
    println!("Your username is {user_name}.");
    println!("You email is {user_email}.");

    // ..user1 is a *move* operation:
    // user1 has partially-moved into user2 for `username`, so we cannot use it as a whole anymore
    // let test_user: User = user1;
    // Error: use of partially moved value: `user1` partial move occurs because `user1.username` has type `String`, which does not implement the `Copy` trait

    // But the email field was not moved out because we specified a different email value for user2
    // So we can still borrow it (or move ownership of it) from the original user1
    let test_useremail: String = user1.email;
    println!("The test email is {test_useremail}.");
    println!();

    println!("Example of Using Tuple Struct");
    println!("-----------------------------");
    let red: Color = Color(255, 0, 0);
    let origin: Point = Point(0, 0, 0);

    // Tuple index access isn't supported in string formatting
    let c_r: i32 = red.0;
    let c_g: i32 = red.1;
    let c_b: i32 = red.2;
    // We can also destruct like tuple, but we must match the struct type
    let Point(origin_x, origin_y, origin_z) = origin;

    println!("The color red is R:{c_r} G:{c_g} B:{c_b}");
    println!("The origin point is ({origin_x}, {origin_y}, {origin_z})");
    println!();
}

// Example of defining a Struct
// ----------------------------
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/// Example of function that returns a struct instance.
///
/// Params:
/// - `email: String` - The email address of the user.
/// - `username: String` - The username of the user.
///
/// Returns:
/// - An instance of User.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // Field Init Shorthand syntax
        username, // username: username,
        email, // email: email,
        sign_in_count: 1,
    }
}

// Example of Tuple Struct
// -----------------------
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

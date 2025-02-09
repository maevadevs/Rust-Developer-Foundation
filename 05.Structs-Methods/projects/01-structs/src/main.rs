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
    let user_name: String = user1.username;
    let user_email: String = user1.email;
    let user_signin_count: u64 = user1.sign_in_count;

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
    let user_name: String = user1.username;
    let user_email: String = user1.email;

    println!("Your username is {user_name}.");
    println!("You email is {user_email}.");
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

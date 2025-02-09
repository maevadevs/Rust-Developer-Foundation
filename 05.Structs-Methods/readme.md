# Structs and Methods

---

- [Defining and Instantiating Structs](#defining-and-instantiating-structs)
  - [Struct Instantiation Is An *Expression*](#struct-instantiation-is-an-expression)
  - [Creating Instances from Other Instances with Struct Update Syntax](#creating-instances-from-other-instances-with-struct-update-syntax)

---

- **Struct**
  - Custom data type, similar to an *Object's Data Attributes* in OOP languages
  - Allows to *package-together* and *name* multiple related values to make-up a meaningful group
  - One of the building blocks for creating new types in Rust
  - Takes full advantage of Rust’s compile-time type checking
- **Methods**
  - Associated functions
  - Specify behavior associated with a `struct` type

## Defining and Instantiating Structs

- Structs are similar to Tuples, but more flexible
  - *Hold multiple related values*
  - The pieces of a Struct can be different types
  - However, we need to name each piece of data in a Struct so it is clear what the values mean
  - *Unlike Tuples, we do not have to rely on the order of the data to specify or access the values of an instance*
- Use the `struct` keyword to define
  - The struct name should describe the significance of the pieces of data being grouped together
  - Each *field* is defined as the name and type of the pieces of data
  - **The fields are unordered**

```rs
// Example of defining a Struct
// ----------------------------
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

- **To use a Struct, we *instantiate* from it**
  - Specify concrete values for each of the fields
  - State the name of the Struct and add curly brackets containing `key: value` pairs
  - Keys are the names of the fields
  - Values are the data we want to store in those fields
  - *The order of the fields does not matter*
- To access a specific field, we use the `.` operator
  - **NOTE: We cannot use *field access expression* using `.` directly in string interpolations**
  - Pull into a variable first, then use in string interpolation
- If the instance is mutable, we can modify the field's value using `.` operator
  - **NOTE: To allow mutability, the entire instance must be mutable**
  - We cannot mark *only certain fields* as mutable

```rs
fn main() {
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
}
```

### Struct Instantiation Is An *Expression*

- It returns a new struct instance as value
- We can use it as the return value of a function
- It usually makes sense to name the function parameters the same as the struct fields
  - ***Field Init Shorthand* syntax: When the function parameter names and the struct field names are exactly the same, we can use simply specify the names when instantiating the struct**

```rs
fn main() {
    println!("Example of Returning a Struct From a Function");
    println!("---------------------------------------------");

    let user1: User = build_user("dummy@email.com", "anonymous123");
    let user_name: String = user1.username;
    let user_email: String = user1.email;

    println!("Your username is {user_name}.");
    println!("You email is {user_email}.");
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
```

### Creating Instances from Other Instances with Struct Update Syntax

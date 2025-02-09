# Structs and Methods

---

- [Defining and Instantiating Structs](#defining-and-instantiating-structs)
  - [Struct Instantiation Is An *Expression*](#struct-instantiation-is-an-expression)
  - [Creating Instances from Other Instances with *Struct Update Syntax*](#creating-instances-from-other-instances-with-struct-update-syntax)
  - [Using Tuple Structs Without Named Fields to Create Different Types](#using-tuple-structs-without-named-fields-to-create-different-types)
  - [Unit-Like Structs Without Any Fields](#unit-like-structs-without-any-fields)

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
- **Each `struct` we define is its own type**
  - Even though the fields within the structs might have the same types
  - A function that takes a parameter of type `struct Color` cannot take a `struct Point` as an argument, even if they are both made up of three `i32` values

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
  - Use *borrow* so the ownership does not *move*
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
    // NOTE: The Struct needs to be mutable for this to work
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
}
```

### Struct Instantiation Is An *Expression*

- It returns a new struct instance as value
- We can use it as the return value of a function
- It usually makes sense to name the function parameters the same as the struct fields
  - ***Field Init Shorthand* syntax: When the function parameter names and the struct field names are exactly the same, we can simply specify the names when instantiating the struct**

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
```

### Creating Instances from Other Instances with *Struct Update Syntax*

- It is often useful to create a new instance of a struct that includes most of the values from another instance
  - Then we apply some changes to customize the new instance
  - We can do this with *Struct Update Syntax*
- We can specify the fields that we want to have different values
  - Then, for the remaining fields, we grab the same values as from another instance
  - `..` means the fields not explicitly set will have the same value as the fields in the given instance
  - **It must come last to default the remaining fields to the instance's values**

```rs
fn main() {
    println!("Example of Struct Update Syntax");
    println!("-------------------------------");

    let user2: User = User {
        // Set the email as different value
        email: String::from("another@example.com"),
        // And everything else is the same as in user1
        ..user1
    };

    // Use *borrow* so the ownership does not *move*
    let user_name: &String = &user2.username;
    let user_email: &String = &user2.email;
    println!("Your username is {user_name}.");
    println!("You email is {user_email}.");
}
```

- **NOTE: The *Struct Update Syntax `..`* uses `=` like an assignment**
  - This is a *partial-move* operation
  - The defaulting fields `String` (`username`) from the original instance moved ownership into the new instance
  - But the non-defaulting fields `String` (`email`) did not move out because a new value was specified for it
  - After the operation, the original instance is no longer accessible *as a whole*
  - **This is caused mainly by `Heap` fields: The `Stack` fields `bool` and `u64` implement the *Copy* trait**

```rs
let user2: User = User {
    // Set the email as different value
    email: String::from("another@example.com"),
    // And everything else is the same as user1
    ..user1
};

// ..user1 is a *move* operation:
// user1 has partially-moved into user2 for `username`, so we cannot use it as a whole anymore
// let test_user: User = user1;
// Error: use of partially moved value: `user1` partial move occurs because `user1.username` has type `String`, which does not implement the `Copy` trait

// But the email field was not moved out because we specified a different email value for user2
// So we can still borrow it (or move ownership of it) from the original user1
let test_useremail: String = user1.email;
println!("The test email is {test_useremail}.");
```

- **The non-`String`, primitive fields are unaffected because they implement the `Copy` trait (Stack values)**

### Using Tuple Structs Without Named Fields to Create Different Types

- *Tuple Structs*
  - Structs that look similar to tuples
  - We can destructure them into their individual pieces (but we must match the struct type)
  - They are supported in Rust
  - Have the added meaning the `struct` name provides
  - *But do not have names associated with their fields: They just have the types of the fields*
  - **We can think of them as *Named Tuples***
- Useful when we want to give the whole tuple a name
  - Also makes the tuple a different type from other tuples
  - Can be more flexible than a struct when naming each field in a regular struct is verbose or redundant
- To define, we use the keyword `struct` with specified tuple types

```rs
struct TupleStructName(<fieldType>, <fieldType>, <fieldType>, ...)
```

```rs
// Example of Tuple Struct
// -----------------------
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    println!("Example of Using Tuple Struct");
    println!("-----------------------------");
    let red: Color = Color(255, 0, 0);
    let origin: Point = Point(0, 0, 0);

    // Tuple index access isn't supported in string formatting
    let c_r: i32 = red.0;
    let c_g: i32 = red.1;
    let c_b: i32 = red.2;
    // We can also destruct like tuple, but we must match the type
    let Point(origin_x, origin_y, origin_z) = origin;

    println!("The color red is R:{c_r} G:{c_g} B:{c_b}");
    println!("The origin point is ({origin_x}, {origin_y}, {origin_z})");
}
```

### Unit-Like Structs Without Any Fields

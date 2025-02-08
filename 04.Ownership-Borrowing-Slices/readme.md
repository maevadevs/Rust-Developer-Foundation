# Ownership, Borrowing, and Slices

---

- [What Is Ownership](#what-is-ownership)
  - [Stack and Heap](#stack-and-heap)
    - [Stack](#stack)
    - [Heap](#heap)
    - [Stack vs Heap](#stack-vs-heap)
  - [Ownerhsip Rules](#ownerhsip-rules)
  - [Variable Scope](#variable-scope)
  - [`String` Type](#string-type)
  - [Memory and Allocation](#memory-and-allocation)
    - [Variables and Data Interacting with *Move*](#variables-and-data-interacting-with-move)
    - [Variables and Data Interacting with *Clone*](#variables-and-data-interacting-with-clone)
    - [Stack-Only Data: *Copy*](#stack-only-data-copy)
  - [Ownership and Functions](#ownership-and-functions)
  - [Return Values and Scope](#return-values-and-scope)
- [References and Borrowing](#references-and-borrowing)
  - [Mutable Reference](#mutable-reference)
  - [Dangling Reference](#dangling-reference)
  - [Rules of References](#rules-of-references)
- [The `Slice` Type](#the-slice-type)
  - [Example of a Problem](#example-of-a-problem)
  - [Solving Problem Without Using `Slice`](#solving-problem-without-using-slice)
  - [Solving Problem Using `String Slice`](#solving-problem-using-string-slice)
    - [NOTE: Range Syntax](#note-range-syntax)
    - [Rewriting `first_word()` With Slice](#rewriting-first_word-with-slice)
  - [String Literals As Slices](#string-literals-as-slices)
  - [String Slices As Parameters](#string-slices-as-parameters)
  - [Other Slices](#other-slices)
  - [Slice Similarities In Other Languages](#slice-similarities-in-other-languages)

---

- *Ownership*, *Borrowing*, and *Slices* ensure memory safety in Rust programs at compile time
- *Ownership* is Rust's most unique feature
  - Has deep implications for the rest of the language
  - **Allows to make memory-safety guarantees without needing a GC**
  - Affects how lots of other parts of Rust work
- Other related features:
  - Borrowing
  - Slices
  - How Rust lays out data in memory

## What Is Ownership

- **A set of rules that governs how a Rust program manages memory**
  - **Memory is a finite resource**: All programs must manage memory efficiently while running
  - Some languages use automation via [GC](https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)) or [ARC](https://en.wikipedia.org/wiki/Reference_counting) to clear memory during runtime: Go, Python, C#, Java, ECMAScript, Swift
  - Other languages must have their memory managed manually and explicitly ahead-of-time: C, C++, Pascal, Fotran, Zig
- Rust uses a different approach
  - **Memory is managed through a *System of Ownership***
  - **There are a set of rules that the compiler checks**
  - **If any of the rules are violated, the program will not compile**
  - **None of the features of ownership will slow down the program during runtime**
- Understanding *Ownership* gives a solid foundation for understanding the features that make Rust unique

### Stack and Heap

- **In system programming language, understanding Stack and Heap is essential**
  - Whether a value is on the Stack or the Heap affects how the language behaves
  - Also affects why we have to make certain decisions
  - *Parts of Ownership is described in relation to the Stack and the Heap*
- **Stack and Heap are parts of memory available during runtime**
  - But they are structured in different ways
  - Used for different purposes

#### Stack

- **Last-In, First-Out (LIFO) data structure**
  - Stores values in the order it gets them
  - Removes the values in the opposite order
  - *Similar to a stack of plates*
- Adding/Removing from the middle would not work well
  - Adding data == *Pushing* onto the Stack
  - Removing data == *Popping* from the Stack
- **All data stored on the Stack must have a known-in-advanced type and fixed-size**
  - Else, store the data in the Heap instead

#### Heap

- Less organized than the Stack
- **The memory allocator finds an empty spot in the Heap that is big enough**
  - Mark it as *being used* and store the value there
  - Return a *pointer/address* to that location
  - This process is called *Allocating Memory* on the Heap
  - *NOTE: Pushing value unto the Stack is not considered Allocating*
- **The pointer to the Heap is a known and is a fixed-size value (Address)**
  - *This pointer is stored on the Stack as a reference to the Heap location*
  - When we want the actual data, we must follow the pointer to the Heap location

#### Stack vs Heap

- **Pushing unto the Stack is faster than allocating unto the Heap**
  - The allocator never has to search for a place to store new data in the Stack
  - Location is always at the top of the Stack
  - For Heap, the allocator must first find a big-enough space to hold the data
  - Then perform bookkeeping to prepare for the next allocation
- **Accessing data from the Heap is slower than from the Stack**
  - Have to follow a pointer to get to the value
  - Processing is faster if we don't jump around (Stack)
  - A processor is better if it works on data that is closer to other data
- **When calling a function, the arguments and parameters get pushed onto the Stack**
  - The parameters can potentially be pointers to Heap values
  - The function's local variables are also pushed onto the Stack
  - When the function is over, those values get popped off the Stack
- ***The Main purpose of Ownership is to manage data in Heap memory***
  - Keep track of what parts of code are using what data on the Heap
  - Minimize amount of duplicate data on the Heap
  - Clean up unused data on the Heap

### Ownerhsip Rules

- **Each value in Rust has a dedicated owner**
- **Value-ownership can be *transferred* to a different owner**
- **Values can be *borrowed* from the owner**
- **There can only be one owner of a value at a time**
- **When the owner goes out of scope, its owned values will be dropped**

### Variable Scope

- **A *scope* is the range within a program for which an item is valid**
- A variable is valid from the point at which it is declared until the end of the current scope
- **NOTE: Rust is a *block-scoped* language**
  - We can create a block with `{}`
  - `{}` blocks can also be standalone

```rust
{
    let greetings: &str = "hello";
    // greetings is valid in this block from this point forward
    // Do stuff with greetings here
}
// This scope is now over, and greetings is no longer valid
```

- There are 2 important points in time
  - When the variable comes into scope, it is valid
  - It remains valid until it goes out of scope

### `String` Type

- A data type that is more complex than the primitives
- Previous Data Types so far are of known-size (fixed-size)
  - *Integers*, *Floats*, *Booleans*, *Tuples*, *Arrays*
  - Stored on the Stack
  - Popped off the Stack when their scope is over
  - Can be quickly and trivially copied to make a new independent instance (copied by value)
- **String literals `&str` are *immutable***
  - Allow to hard-code string values
  - Not suitable for every situation in which we may want to use text
  - Not every string length is known at compile-time
- **`String` is a *mutable* data type of dynamic length**
  - Stored on the Heap
  - Can store an amount of text that is unknown at compile time
  - We can explore it to learn how Rust cleans-up data in Heap memory
  - Can be created using the `String::from(&str)` function
- For now, we concentrate on the parts of `String` that relate to *Ownership*
  - Concepts here also applies to other complex Heap data types

```rs
// Example of Creating a String
// ----------------------------
// NOTE: String can be mutated
println!("Example of Creating a String:");
println!("-----------------------------");

let mut st: String = String::from("Hello World!");
// push_str() appends a literal to a String (Mutating)
st.push_str(" ");
st.push_str("Hello everyone!");
println!("st = {st}");
```

- `::` operator allows to *namespace* a particular function under the type
  - Better than using some sort of name like `string_from()`
- `st.push_str()` appends a literal to a `String`, mutating `st`
- **The difference between `String` and string literals `&str` is how they deal with memory**

### Memory and Allocation

- **With string literals, we know the contents and length at compile-time**
  - The text is hardcoded into the final executable
  - Fast and efficient
  - Known length at compile-time
  - Immutable
- **We cannot do the same for string with unknown contents at compile-time**
  - Size might change during program execution
- **`String` supports growable and mutable text that has unknown contents and is not fixed at compile-time**
  - Allocate memory on the Heap
  - Memory is requested from the memory allocator at runtime
  - Memory should be returned to the allocator when finished with the string
- Memory allocation is done with `String::from()`
- ***Memory deallocation***
  - In GC-based languages, this would be handled by the GC
  - Without GC, we need to tell when to free the memory
  - Doing this correctly manually without GC has always been challenging in languages like C/C++
  - Need to pair exactly one `allocate` with exactly one `deallocate`
- Rust uses a different approach
  - **The memory is automatically freed once the variable that *owns* it goes out of scope**

```rs
{
    let st: String = String::from("hello");
    // st is valid from this point forward
}
// The scope is now over, and st is no longer valid
// The memory used by the value stored in st is now deallocated
```

- **When a variable goes out of scope, Rust calls a special function `drop()`**
  - `drop()` defines how a `String` gets deallocated from memory
  - Rust calls `drop()` automatically at the closing curly bracket (end of scope)
- **NOTE: In C++, this pattern is called *Resource Acquisition Is Initialization (RAII)***
  - This pattern has a profound impact on the way Rust code is written
  - The behavior of code can be unexpected in more complicated situations
  - E.g. When we want to have multiple variables to use the data allocated on the Heap

#### Variables and Data Interacting with *Move*

- Multiple variables can interact with the same data in different ways

```rs
// Bind the value 5 to x
let x: i32 = 5;

// Make a copy of the value in x and bind it to y
let y: i32 = x;
```

- Integers are simple values with a known fixed size
- They are **copied-by-value** to a different variable
- Each value is pushed independently unto the Stack

```rs
// Bind the value "hello" to s1
let s1: String = String::from("hello");

// This does not make a separate copy of s1
let s2: String = s1;
```

- A `String` is made of 3 parts
  - A *pointer* - Points to the Heap memory address that actually holds the contents of the string
  - A *length* - How much memory in bytes the contents of the `String` are *currently* using
  - A *capacity* - Total amount of memory in bytes that the `String` has received from the *Allocator*
- **This group of data is what is stored on the Stack under the variable**
  - The actual contents of the string is held in the Heap
  - The *pointer* points to the memory address of the contents in the Heap

<img src="./img/String-In-Memory.png" width="30%" />

- **Reassigning the `String` to another variable only copies the *pointer*, *length*, and *capacity* to the Stack**
  - The Heap data itself is not copied
  - Else, the operation would be too expensive, especially when the Heap data is large
- **When a Heap variable goes out of scope, Rust automatically calls the `drop()` function**
  - Cleans up the Heap memory for that variable

<img src="./img/String-Variable-Alias.png" width="30%" />

- **What happen when multiple variables point to the same value in Memory (`s1` and `s2`)?**
  - When `s2` and `s1` go out of scope, they will both try to free the same pointed memory
  - **This is known as a *double-free* error**
  - **Freeing memory twice can lead to memory corruption**
  - It can potentially lead to security vulnerabilities
- **To ensure memory safety, after the line `let s2 = s1;`, Rust considers `s1` as no longer valid**
  - **This is known as a *Move*: `s1` was *moved* into `s2`**
  - Rust does not need to free anything when `s1` goes out of scope
  - **Using `s1` after `let s2 = s1;` will result in an error**
  - Rust prevents using the invalidated reference

<img src="./img/Move.png" width="30%" />

```rs
// Example of Variables and Data Interacting with *Move*
// -----------------------------------------------------
println!("Example of Variables and Data Interacting with *Move*");
println!("-----------------------------------------------------");

// Bind the value "hello" to s1
let s1: String = String::from("hello");
// This does not make a separate copy of s1
// Only copy the pointer, length, and capacity
// But moves the ownership of the data in memory from s1 to s2
// And invalidates s1
let s2: String = s1;

// println!("s1 = {s1}"); // error[E0382]: borrow of moved value: `s1`
println!("s2 = {s2}");
```

- With only `s2` valid, when it goes out of scope, it alone will free the memory
  - This solves the *double-free* error
- There is also a *design choice* on Rust
  - **Rust will never automatically create deep-copies of data**
  - Any automatic copying can be assumed to be inexpensive in terms of runtime performance
- **NOTE: When assigning a completely new value to an existing variable, Rust will call `drop()` and free the original value’s memory immediately**
  - Does not wait at the end of the scope

```rs
let mut s = String::from("hello");
s = String::from("ahoy"); // Here, the original "hello" is immediately dropped from memory
println!("{s}, world!");
```

#### Variables and Data Interacting with *Clone*

- `.clone()` allows to make a *deep-copy* of the data in Heap *by value*
- **The data value in Heap itself gets copied/cloned: This can be expensive with large data**
- Cloning does not *move* the value so both variables are still valid after the operation

```rs
// Example of Variables and Data Interacting with *Clone*
// ------------------------------------------------------
println!("Example of Variables and Data Interacting with *Clone*");
println!("------------------------------------------------------");

let s1: String = String::from("hello");
let s2: String = s1.clone();

println!("s1 = {s1}, s2 = {s2}");
```

#### Stack-Only Data: *Copy*

- **Primitive Types that have a known-size at compile-time are stored entirely on the Stack**
  - Copies of the actual values are quick to make
  - There is no difference between *deep* and *shallow* copying here
  - **The value is always copied *by value***
  - Calling `.clone()` would not do anything different

```rs
// Bind the value 5 to x
let x: i8 = 5;

// Make a copy of the value in x and bind it to y
let y: i8 = x;

// Both variable are still available
println!("x = {x}");
println!("y = {y}");
```

- **Rust has a special annotation `Copy` trait that we can place on types that are stored on the Stack**
  - For any type that implements the `Copy` trait, variables that use it do not *Move*
  - Instead, they are trivially copied, making them still valid after assignment to another variable
  - This is the case of the Primitive Stack Types
- **Rust does not allow annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait**
  - Result into a compile-time error
- **Types that implement the `Copy` trait**
  - *Can implement*: As a general rule, any group of simple scalar values
  - *Cannot implement*: Anything that requires allocation (Heap) or is some form of resource
  - Some of the types that implement `Copy`:
    - Integers
    - Boolean
    - Floating-points
    - Character
    - Tuples, but only if they strictly contain types that also implement `Copy`

### Ownership and Functions

- **Passing a value to a function is similar to assigning a value to a variable**
  - *Move* or *Copy*, just as assignment does

```rs
// Example of Ownership With Functions
// -----------------------------------

fn main() {
    // Example of Ownership With Functions
    // -----------------------------------
    println!("Example of Ownership With Functions");
    println!("-----------------------------------");

    let st: String = String::from("hello");  // st comes into scope

    takes_ownership(st);            // st's value moves into the function
    // println!("st = {st}");       // so st is no longer valid here: This causes an error

    let x: i32 = 5;                 // x comes into scope

    makes_copy(x);                  // x would move into the function,
    println!("x = {x}");            // but i32 is Copy-Only, so it's okay to still use x afterward
}
// Here, x goes out of scope, then st.
// But because st's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("some_string = {some_string}");
}
// Here, some_string goes out of scope and `drop` is called.
// The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("some_integer = {some_integer}");
}
// Here, some_integer goes out of scope.
// Nothing special happens.
```

- Trying to use `st` after the call `takes_ownership(st)` would throw a compile-time error
  - These static checks protect us from mistakes

### Return Values and Scope

- **Returning values can also transfer ownership**
- The ownership of a variable follows the same pattern every time
- Assigning a value to another variable moves it
- When a variable that includes data on the Heap goes out of scope, the value will be cleaned up by `drop()`
  - Unless ownership of the data has been moved to another variable

```rs
// Example of Return Values and Ownership
// --------------------------------------

fn main() {
    println!("Example of Return Values and Ownership:");
    println!("---------------------------------------");

    let s1: String = gives_ownership();         // gives_ownership() moves its return value into s1
    let s2: String = String::from("hello");     // s2 comes into scope
    let s3: String = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back()
                                                // which also moves its return value into s3

    println!("s1 = {s1}");
    // println!("s2 = {s2}"); // s2 has moved
    println!("s3 = {s3}");
}
// Here, s3 goes out of scope and is dropped
// s2 was moved, so nothing happens
// s1 goes out of scope and is dropped

/// gives_ownership() will move its return value into the function that calls it
fn gives_ownership() -> String {
    let some_string: String = String::from("yours"); // some_string comes into scope
    return some_string; // some_string is returned and moves out to the calling function
}

// takes_and_gives_back() takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    return a_string;  // a_string is returned and moves out to the calling function
}
```

- **Taking ownership and then returning ownership with every function is a bit tedious**
  - What if we want to let a function use a value but not take ownership?
  - It is quite annoying that anything we pass in needs to be passed back if we want to use it again
  - We could resolve this by returning multiple values using a tuple

```rs
fn main() {
    let s1: String = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(st: String) -> (String, usize) {
    let length: usize = st.len(); // len() returns the length of a String
    return (st, length);
}
```

- But this is too much ceremony and a lot of work for a concept that should be common
  - We have to return the `String` to the calling function so we can still use it after the call
  - The `String` was moved into `calculate_length()` and needs to be returned back for further usage
- ***Reference* - A feature for using a value without transferring ownership**

## References and Borrowing

- In the example above, we can provide a reference to the `String` value
- *Reference*
  - Similar to *Pointer*
  - An address we can follow to access the data stored at that address
  - The data is owned by a different variable
  - *Unlike a Pointer, a Reference is **guaranteed to point to a valid value of a particular type for the life of that reference***
  - ***A reference's scope starts from where it is introduced and continues through the last time that reference is used, or the end of the block, whichever is first***

```rs
fn main() {
    println!("Example of Borrowing With Reference:");
    println!("------------------------------------");

    let s1: String = String::from("hello");
    let len: usize = calculate_length(&s1); // s1 is borrowed by the function via reference
    println!("The length of '{s1}' is {len}."); // s1 was not moved: Still available here
}

/// A function that borrows a reference
fn calculate_length(s: &String) -> usize {
    return s.len();
}
```

- `calculate_length()` takes a `&String` and is passed `&s1` a reference to `s1`
  - Ampersands `&` represent references
  - **Allows to refer to some value without taking ownership of it**
  - We also pass reference to the function when we call it

<img src="./img/Reference-And-Borrowing.png" width="50%" />

- **NOTE: The opposite of *referencing* with `&` is *dereferencing* with `*`**
  - `&s1` creates a reference that refers to the value of `s1` ***but does not own it***
  - **Without ownership, the value it points to will not be dropped when the reference stops being used**
- **The signature of the function uses `&` to indicate that the type of the parameter `s` is a reference**
  - The scope in which the variable `s` is valid is the same as any function parameter's scope
  - But the value pointed to by the reference is not dropped when `s` stops being used
  - `s` does not have ownership: Just borrowing
- **When functions have references as parameters, no need to return the values in order to give back ownership**
  - We never had ownership to begin with
- **Borrowing**
  - The action of creating a reference
  - As in real life, if a person owns something, you can borrow it from them
  - When you are done, you have to give it back because you do not own it
- **Trying to modify a borrowed value will create a compile-time error**
  - **Similar to variables, by default, references are *immutable***
  - But we can specify it to be mutable as well

```rs
fn main() {
    let s: String = String::from("hello");
    change(&s); // Attempting to change a borrowed value
}

/// A borrowing function.
fn change(some_string: &String) -> () {
    // Attempting to change a borrowed value
    some_string.push_str(", world"); // error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
}
```

### Mutable Reference

- We can change the function parameters references to be mutable using `&mut`
  - This makes it very clear that the function will mutate the value of the reference it borrows
- **But the target variable itself also needs to be `mut`**
  - This allows the borrowed value to be modified

```rs
fn main() {
    println!("Example of Mutable Reference");
    println!("----------------------------");

    // The target variable needs to be mutable
    let mut s2: String = String::from("hello");
    println!("Before: s2 = {s2}");
    change_str(&mut s2); // Attempting to change a mutable borrowed value
    println!("After: s2 = {s2}");
}

/// A borrowing function: Parameter is mutable reference.
fn change_str(some_string: &mut String) -> () {
    some_string.push_str(", world");
}
```

- **One big restriction of mutable references: *If you have a mutable reference to a value, you can have no other references to that value***
  - A value cannot have more than 1 mutable reference at a time
  - Only a single mutable reference is allowed at a time
  - **This allows Rust to prevent data races at compile time**
  - In real-world, it is the same: An object can only be borrowed once at a time, especially if it needs to be modified

```rs
let mut st: String = String::from("hello");

let ref1: &mut String = &mut st;
let ref2: &mut String = &mut st; // => error[E0499]: cannot borrow `s` as mutable more than once at a time
let ref3: &String = &st; // error[E0502]: cannot borrow `st` as immutable because it is also borrowed as mutable

println!("{ref1}, {ref2}");
```

- A *Data Race* happens when 3 behaviors occur:
  - 2 or more pointers access the same data at the same time
  - At least 1 pointer is used to write to the data
  - There is no mechanism used to synchronize access to the data
- Data Races cause undefined behaviors
  - Can be difficult to diagnose and fix at runtime
  - Rust refuses to compile codes with Data Races
- **Multiple mutable references can exist in different scopes**
  - But they must not happen simultaneously

```rs
let mut st: String = String::from("hello");

// Separate scope
{
    let r1: &mut String = &mut st;
}
// r1 goes out of scope here
// So we can make a new reference with no problems
let r2: &mut String = &mut st;
```

- **NOTE: We cannot combine mutable and immutable references**
  - We also cannot have a mutable reference while we have an immutable one to the same value
  - This allows users of an immutable reference not to expect the value to suddenly change out
- **Multiple immutable references are allowed**
  - No one who is just reading the data has the ability to affect anyone else reading the same data
- **Mutable reference to an immutable variable is not allowed**
- **NOTE: A reference's scope starts from where it is introduced and continues through the last time that reference is used, or the end of the block, whichever is first**
  - These scopes do not overlap
  - The compiler can tell that the reference is no longer being used at a point before the end of the scope

```rs
{
    let mut st: String = String::from("hello");

    let mut_r1: &mut String = &mut st; // 1st Mutable Reference
    // let mut_r2: &mut String = &mut st; // 2nd Mutable Reference: Error: Cannot borrow a 2nd time

    println!("{mut_r1}");
    println!("{mut_r1}"); // Last time the 1st Mutable Reference is used within this block. End of scope for mut_r1

    let mut_r2: &mut String = &mut st; // 2nd Mutable Reference: We can now borrow as 1st mutable reference scope has ended
    println!("{mut_r2}");
}
```

### Dangling Reference

- A pointer that references a location in memory that may have been given to someone else
  - I.e. an invalid pointer
  - Created by freeing some memory while preserving a pointer to that memory
  - Can be easy to create in language that uses pointers
- **Rust compiler guarantees that references will never be dangling**
  - *Unlike a Pointer, a Reference is **guaranteed to point to a valid value of a particular type for the life of that reference***
  - The compiler ensures that the data will not go out of scope before the reference to the data does

```rs
fn main() {
    let ref_to_nothing: &String = dangle();
}

fn dangle() -> &String {
    // s is a new String
    let s: String = String::from("hello");
    // we return a reference to the String s
    &s
}
// Scope of s ends here so &s points to nothing (dangling)
```

- Compiling the above snippet creates an error
  - `error[E0515]: cannot return reference to local variable s. This function's return type contains a borrowed value, but there is no value for it to be borrowed from`
  - This is related to Rust feature called *Lifetimes*
- Because `s` is created inside `dangle()`, when the code of `dangle()` is finished, `s` will be deallocated
  - The returned reference would be pointing to an invalid `String`
  - The solution is to return the String directly, passing ownership to the caller (not borrowing)
  - When ownership is moved out, nothing is deallocated

```rs
fn no_dangle() -> String {
    // s is a new String
    let s: String = String::from("hello");
    // Return s and pass its ownership along to the caller
    s
}
// End of scope for no_dangle:
// Ownership of s is moved out and nothing is deallocated
```

### Rules of References

- **At any given time, you can have either one mutable reference or any number of immutable references per scope**
- References must always be valid
- A reference's scope starts from where it is introduced and continues through the last time that reference is used, or the end of the block, whichever is first

## The `Slice` Type

- Allows to reference a *contiguous sequence* of elements in a collection
  - Instead of the whole collection
  - i.e. it is like a window within the collection
  - **A Slice is a reference: no an ownership**
- **NOTE: *Collections***
  - Data structures that can contain multiple values
  - Part of Rust's Standard Library
  - Data is stored on the Heap
  - The amount of data does not need to be known at compile time
  - Can grow or shrink as the program runs
  - Most used collections in Rust:
    - *Vector* - Store a variable number of values next to each other
    - *String* - Collection of characters
    - *Hash Map* - Associate a *value* with a specific *key*

### Example of a Problem

- Write a function that takes a string of words separated by spaces
  - Return the first word it finds in that string
  - If there is no space in the string, return the entire string
  - E.g. `Hello World!` => `Hello`
  - E.g. `Anonymous` => `Anonymous`

### Solving Problem Without Using `Slice`

```rs
fn first_word(st: &String) -> ?
```

- We do not need ownership of the String: Just borrowing
- But what should be returned? *Part* of a String?
- We could return the index of the end of the word

```rs
fn main() {
    // Example of Solving Problem Without Using Slice
    // ----------------------------------------------
    // NOTE: String can be mutated
    println!("Example of Solving Problem Without Using Slice:");
    println!("-----------------------------------------------");

    let hello: usize = first_word(&String::from("Hello World!"));
    println!("Hello World! -> {hello}"); // => 5

    let abraca: usize = first_word(&String::from("Abracadabra!"));
    println!("Abracadabra! -> {abraca}"); // => 12
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
```

- We convert the `String` to an array of `bytes`: `st.as_bytes()`
- We iterate over the array of `bytes`: `bytes.iter().enumerate()`
  - `iter()` returns each element in a collection
  - `enumerate()` wraps the result of `iter()` and returns each element as a `tuple` of `(index, &byte)`
  - We use *tuple pattern destructuring* with the `for` loop
  - The resulting element is a reference so we use `&`
  - *NOTE: These approaches are familiar to Python*
  - In the loop, we search for the byte representing `' '` using a *byte-literal* syntax `b' '`
- **Issue with this approach**
  - We are returning a `i: usize` index on its own
  - ***That is only a meaningful number in the context of the `&String`, as in `&String[i: usize]`***
  - Because it is a separate value from the `&String`, we cannot guarantee that it will still be valid in the future

```rs
fn main() {
    let mut st: String = String::from("hello world");

    // word will get the value 5
    let word: usize = first_word(&st);

    // st is mutable
    // So this empties the String to ""
    st.clear();

    // word still has the value 5 here
    // but there is no more string that we could
    // meaningfully use the value 5 with.
    // word[5] no longer makes sense!
    // word is now totally invalid!
}
```

- The program compiles without error
  - For the compiler, `word` is *not connected to the state of `st` at all*
  - Using `word` as index on `st` would cause a runtime bug
- **Having to worry about an index getting out of sync with the data is tedious and error prone**
  - What if we also want to return the index of the second word, Third word,... as tuple?
  - We might need to return the `start` and `end` indices
  - This can get complicated fast: *We have multiple unrelated variables floating around that need to be kept in sync*
- The better solution is to use `Slice`

### Solving Problem Using `String Slice`

- A *String Slice* is a *reference* to *part* of a `String`
- *NOTE: This is a familiar approach to Python and Go*

```rs
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
```

- `hello`, `again`, and `world` are references to a portion of the `String`
- **We create *String Slices* using a *range of index* on the `String`**
  - `&String[start...end]`
  - `start` index is inclusive
  - `end` index is exclusive (up-to-but-not-including)
- **Internally, the `Slice` data structure stores `start` and `slice_length`**
  - $SliceLength=end-start$
  - **The `Slice` is a *pointer* to the `start` index of the original `String` (`s`), and for a specific `slice_length`**
- **We can think of *Slice* as a *window* to look through part of the String**

<img src="./img/String-Slice.png" width="25%" />

#### NOTE: Range Syntax

- The range syntax allows to drop the `start` or `end` to mean *from the start* or *to the end*
- We can also drop both value to mean *grab everything*

```rs
let st: String = String::from("hello");

// These syntax are equivalent: From the start
let slice: &str = &st[0..2];
let slice: &str = &st[..2];

// These syntax are equivalent: To the end
let slice = &st[3..st.len()];
let slice = &st[3..];

// These syntax are equivalent: Grab everything
let slice = &st[0..st.len()];
let slice = &st[..];
```

- **NOTE: String slice range indices must occur at valid UTF-8 character boundaries**
  - Attempting to create a string slice in the middle of a multibyte character will create an runtime error

```rs
// Example of Slicing at Non-UTF-8 Character Boundary
// --------------------------------------------------
println!("Example of Slicing at Non-UTF-8 Character Boundary:");
println!("---------------------------------------------------");

let st: String = String::from("Hello ❤️!");
let st_len: usize = st.len();
println!("{st_len}"); // 13

// Attempting to index in the middle of the ❤️ character
let bad_utf_index: &str = &st[7..10];
println!("{bad_utf_index}");
// Error: byte index 7 is not a char boundary; it is inside '❤' (bytes 6..9) of `Hello ❤️!`
```

#### Rewriting `first_word()` With Slice

- Rewriting `first_word()` to return a `String` slice
- Same approach as before
- However, when we find a space, we return a string slice using the `start` of the string as starting index and the index of the space as ending index
- We get back a single value that is *tied to the underlying data*
  - Under the hood, the value is made up of *a reference to the starting point of the slice* and *the slice length*

```rs
fn main() {
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
}

/// Get the first word of a string,
/// or the original string if it is a single word.
fn first_word_slice(st: &String) -> &str {
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
```

- Same approach can be made for `second_word_slice()`, `third_word_slice()`... with no further complications
  - Straightforward API that is much harder to mess up
  - The compiler ensures that the references into the `String` remain valid
  - The earlier bug involving `st.clear()` would be caught during compile-time

```rs
fn main() {
    let mut st: String = String::from("hello world");

    // word will get the value "hello"
    let word: &str = first_word_slice(&s);

    // This cause a compile-time error!!
    // The compiler ensures that references into st remain valid
    st.clear();
    // error[E0502]: cannot borrow `st` as mutable because it is also borrowed as immutable

    println!("The first word is: {word}");
}
```

- Borrowing Rule reminder: *If we have an immutable reference to something, we cannot also take a mutable reference for the same*
  - `.clear()` needs a *mutable reference* to truncate the `String`
  - `word` is an *immutable reference* and is used in the following `println!`
  - The immutable reference must still be active at that point
  - So the conflict between `.clear()` and `word` causes a compile-time error

### String Literals As Slices

- **String Literals `&str` are essentially Slices**
  - They are compiled as part of the executable binary
  - A slice pointing to that specific point of the compiled executable binary
- This is also why string literals are always immutable
  - **`&str` is an immutable reference to the executable binary**

```rs
let st: &str = "Hello, world!";
```

### String Slices As Parameters

- We can take slices of literals and `String` values
- Both slices would be typed as `&str`
  - Slice of `String` => `&str`
  - Slice of string literal `&str` => `&str`
- **Using `&str` as parameter type for *Slice of string* allows us to use the same function on both `&String` values and `&str` values**
  - If we have a string slice, we can pass that directly
  - If we have a `String`, we can pass a slice of the `String` or a reference to the `String`
  - This flexibility takes advantage of a feature called *Deref Coercion*

```rs
// Change parameter type from &String to &str
fn first_word_slice(st: &str) -> &str {
    ...
}
```

- **Defining a function to take *a string slice* (`&str`) instead of *a reference to a `String`* (`&String`) makes an API more general and useful without losing any functionality**
  - The following examples all works in the same way

```rs
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
```

### Other Slices

- String slices are specific to strings
- There are more general slice type too
- For example, for arrays: `&[<type>]`
- **We can refer to a *slice* of any collections**
  - Work the same way as string slices
  - Storing a reference to the first element and a length

```rs
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
println!("arr_slice == &[20, 30] ? => {is_equal}"); // true
```

### Slice Similarities In Other Languages

- Rust's slice has similar concepts *but not fully-equivalent implementations* with other languages, such as Python and Go
- *However, in Python, the slice is transfered by value, not by reference*
- Also, *Python strings are always immutable*

```py
st: str = "hello world"

# world is not a reference to st but an actual new value in memory
world: str = st[6:11]

print(st)
print(world)
```

- Go's array and slice use *Pointers* and might be closer in concept to Rust's slice and/or vector
  - But differences still applies
- [Check here](https://github.com/maevadevs/Go-Developer-Advanced/tree/main/03-Composite-Types#slices) for more details on how Go makes use of slices

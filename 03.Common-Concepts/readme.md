# Common Programming Concepts

---

- [Variables and Mutability](#variables-and-mutability)
  - [Constants](#constants)
  - [Shadowing](#shadowing)
- [Data Types](#data-types)
  - [Scalar Types](#scalar-types)
    - [Integers](#integers)
      - [Integer Overflow](#integer-overflow)
    - [Floats](#floats)
    - [Notes: Numeric Operations](#notes-numeric-operations)
    - [Booleans](#booleans)
    - [Characters](#characters)
  - [Compound Types](#compound-types)
    - [Tuples](#tuples)
    - [Arrays](#arrays)
      - [Accessing Array Elements](#accessing-array-elements)
- [Functions](#functions)
  - [Parameters](#parameters)
  - [Statements and Expressions](#statements-and-expressions)
  - [Functions With Return Values](#functions-with-return-values)
- [Comments](#comments)
- [Control Flow](#control-flow)
  - [`if-else` Expression](#if-else-expression)
    - [`else if` Expression](#else-if-expression)
    - [Using `if` in `let` Statement](#using-if-in-let-statement)
  - [Repetition With Loops](#repetition-with-loops)
    - [Using `loop`](#using-loop)
    - [Returning Values From Loops](#returning-values-from-loops)
    - [Loop Labels: Disambiguate Between Multiple Loops](#loop-labels-disambiguate-between-multiple-loops)
    - [Conditional Loops with `while`](#conditional-loops-with-while)
    - [Looping Through a Collection with `for`](#looping-through-a-collection-with-for)
- [Project Ideas For Practices](#project-ideas-for-practices)

---

## Variables and Mutability

- **By default, variables in Rust are immutable**
  - Once a value is bound to a *name*, it cannot be changed
  - **Reassigning to an immutable variable is a compile-time error**
  - Immutability ensures safety and easy-concurrency
  - *Mutability can lead to bugs if not managed properly*
  - Cause of bug can be difficult to track down after the fact
  - Immutability makes code easier to reason with
- **Naming Convention: *Use all-lowercase with underscores between words***

```rs
fn main() {
    // Immutable Variable
    // ------------------
    // Variable is immutable by default
    let my_int: i32 = 55;

    println!("Immutable Variable:");
    println!("-------------------");
    println!("immutable my_int = {my_int}");
    println!();

    // Reassigning to an immutable variable is a compile-time error
    // my_int = 6;
    // => error[E0384]: cannot assign twice to immutable variable `my_int`
}
```

- **We can still change variables to be mutable when needed**
  - But we have to ***explicitly*** make a variable mutable
  - **Add `mut` keyword**
  - Explicitly conveys intent that other parts of the code will change this variable
  - Deciding to use mutability is up to you
  - Depends on what you think is clearest in that particular situation

```rs
fn main() {
    // Mutable Variable
    // ----------------
    // Variable is immutable by default
    // But adding keyword `mut` makes it mutable
    let mut mut_int: i32 = 78;

    println!("Mutable Variable:");
    println!("-----------------");
    println!("mutable mut_int = {mut_int}");

    // This line will not generate an error
    mut_int = 1024;

    println!("Now, mutable mut_int = {mut_int}");
}
```

### Constants

- Also designed to be bound once to a name and not change
- **Cannot be set `mut`: Constants are *always* immutable**
- Declare using `const` keyword
  - **Type *must* be annotated**: Constant's types cannot be inferred
  - **Can be declared in any scope**, including the *global* scope
  - Useful to set globally-fixed values that all parts of an app need to know about
- **Can only be set to a *fixed constant expression*, not results of runtime computations**
  - The value of a constant must be determined at compile-time

```rs
// Constants are always immutable
// They are basically always read-only
// Constant values must be determined at compile-time
// Constants can be declared in any scope, including the global scope
const SECONDS_IN_HOUR: u32 = 60 * 60;
const PI: f64 = 3.14159265359;

println!("Examples of Constants:");
println!("----------------------");
println!("SECONDS_IN_HOUR = {SECONDS_IN_HOUR}");
println!("PI = {PI}");
println!();
```

- **Naming Convention: *Use all-uppercase with underscores between words***
- There is a [limited set of expressions](https://doc.rust-lang.org/reference/const_eval.html) that can be used for constants
  - Only a subset of all expressions can be evaluated at compile-time
  - Can make code easier to understand
  - Gives meaning to the value of the derived constant
- **Constants are valid for the duration of the program-run**
  - **But only valid within the scope in which they were declared**
- Useful for storing global values used throughout the app
  - Conveys the meaning of that value to future maintainers of the code
  - Helps to have only one place to change if the hardcoded value need to change

### Shadowing

- **We can declare a new variable with the same name as a previous variable**
  - The first variable is *shadowed* by the second
  - The second variable is what the compiler will see past that point
  - The second variable takes any uses of the variable name to itself
  - **Until either it itself is *shadowed* or the scope ends**
- **NOTE: A scope can be created using standalone block `{}`**

```rs
fn main() {
    let my_int: i32 = 5;
    println!("The value of my_int is: {my_int}");

    // Variable Shadowing and Scope
    // ----------------------------
    // Variable Shadowing is not the same as `mut`
    // Variable Shadowing redeclares the variable with `let`
    // The variable itself does not mutate:
    // We are creating a new variable each time
    let my_int: i32 = my_int + 6;

    println!("Examples of Variable Shadowing and Scopes:");
    println!("------------------------------------------");
    println!("Local-Scope: my_int = {my_int}");
    {
        // In a different scope, this also shadows the same my_int above
        let my_int: i32 = my_int * 2;
        println!("Inside-Scope: my_int = {my_int}");
    }
    // After the scope ends, the shadowing also ends
    // This one is back to the previously-shadowed my_int
    println!("Local-Scope: my_int = {my_int}");
}
```

- **Shadowing is different from marking a variable as `mut`**
  - Compile-time error if we accidentally try to reassign to this variable without using the `let` keyword
  - **The variable itself does not mutate: We are creating a new variable each time**
  - We can apply some transformation on the *value* without affecting the *variable*
- **We can also change the type of the variable while shadowing the same name**
  - Because we are essentially creating a new variable each time anyway
  - *Shadowing allows to reuse the name without a need to create a new variable name*
  - However, the old value is gone (unless in a different scope)

```rs
fn main() {
    println!("Example of Changing Variable Type While Shadowing:");
    println!("--------------------------------------------------");
    let spaces: &str = "   x    ";          // String type: Non-mutable
    println!("Before: spaces = {spaces}");
    let spaces: usize = spaces.len();       // Number type: Non-mutable
    println!("After: spaces = {spaces}");
}
```

- Using `mut` for this will get us a compile error
  - **We are not allowed to mutate a variable's type**

```rs
let mut spaces = "   ";         // String type
// The following will generate an error:
// spaces = spaces.len();       // Number type
// => Cannot mutate a variable's type
```

## Data Types

- Rust is a statically-typed language
  - **Every value in Rust is of a specific data type**
  - **All types of all variables must be known at compile time**
- Compiler can infer the type based on the value
  - *However, when many types are possible (E.g. integers), we must specify a type annotation*

```rs
// Explicit type: Unsigned Integer-32
let guess: u32 = "42".parse().expect("Not a number!");
```

- There are 2 categories of data types
  - *Scalar*
  - *Compound*

### Scalar Types

- Represent single values
- There are 4 primary scalar types
  - *Integers*
  - *Floats*
  - *Booleans*
  - *Characters*

#### Integers

- Number without a fractional component
- Rust has multiple *Integer* types
  - ***Signed Integers* start with `i`**
  - ***Unsigned Integers* start with `u`**
- Signed numbers are stored using *[Two’s Complement](https://en.wikipedia.org/wiki/Two%27s_complement)* representation
- **Default Integer Type: `i32`**

Length|Signed|Unsigned
:-|:-:|:-:
8-bit|`i8`|`u8`
16-bit|`i16`|`u16`
32-bit|`i32`|`u32`
64-bit|`i64`|`u64`
128-bit|`i128`|`u128`
archvar|`isize`|`usize`

- **With $n$ as the length in bit:**
  - **Each *signed variant* can store numbers from $-(2^{n-1})$ to $2^{n-1} - 1$ inclusive**
  - **Each *unsigned variant* can store numbers from $0$ to $2^{n} - 1$ inclusive**
- *`isize` and `usize` depend on the architecture of the computer*
  - 64 bits on 64-bit architecture
  - 32 bits on 32-bit architecture
  - **`isize` and `usize` are useful when indexing some sort of collection**
- **The types of number literals that can be of multiple types can be specified with a suffix**
  - `57u8`
  - `57i32`
- **Number literals can use `_` as visual separators**
  - `1_000_000`
  - `987_654_321`

Supported Integer literals|Examples
:-:|:-
Decimal|`98222`, `98_222`
Hex|`0xff`
Octal|`0o77`
Binary|`0b11110000`, `0b1111_0000`
Byte (`u8` only)|`b'A'`

```rs
// Examples of Signed Integers
// ---------------------------
let byte: i8 = -128;
let short: i16 = -32_768;
let int: i32 = -2_147_483_648;
let long: i64 = -9_223_372_036_854_775_808;
let llong: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;

println!("Example of Signed Integers:");
println!("---------------------------");
println!("i8 = {byte}");
println!("i16 = {short}");
println!("i32 = {int}");
println!("i64 = {long}");
println!("i128 = {llong}");

// Examples of Unsigned Integers
// -----------------------------
let ubyte: u8 = 255;
let ushort: u16 = 65_535;
let uint: u32 = 4_294_967_295;
let ulong: u64 = 18_446_744_073_709_551_615;
let ullong: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;

println!("Example of Unsigned Integers:");
println!("-----------------------------");
println!("u8 = {ubyte}");
println!("u16 = {ushort}");
println!("u32 = {uint}");
println!("u64 = {ulong}");
println!("u128 = {ullong}");
```

##### Integer Overflow

- ***Integer Overflow* is when we assign a value that is larger than the max of the integer type**
- Results in one of 2 behaviors:
  1. In *Debug* mode, checks for integer overflow will cause `panic` at runtime
  2. In *Release* mode, no panic but performs two’s complement wrapping instead: *wrap around* to the minimum of the type
- ***NOTE: Relying on integer overflow’s wrap-around behavior is considered an error***
  - **All possible overlfow should be handled explicitly**

Handling Approach|Description
:-|:-
`wrapping_*` Methods|Wrap in all modes. E.g `wrapping_add()`
`checked_*` Methods|Return the `None` value if there is overflow
`overflowing_*` Methods|Return the value and a boolean indicating whether there was overflow
`saturating_*` Methods|Saturate at the value’s minimum or maximum values

#### Floats

- 2 primitive types for float-values
- **Default Float Type: `f64`**
  - On modern CPUs, roughly the same speed as `f32`
  - But is capable of more precision
- **All floats are signed**
- **Represented according to the IEEE-754 standard**

Length|Type
:-|:-
32-bit|`f32`
64-bit|`f64`

```rs
// Examples of Floats
// ------------------
let max_float32: f32 = f32::MAX;
let min_float32: f32 = f32::MIN;
let max_float64: f64 = f64::MAX;
let min_float64: f64 = f64::MIN;
let max_float32_repr: String = format!("{:+e}", max_float32);
let min_float32_repr: String = format!("{:+e}", min_float32);
let max_float64_repr: String = format!("{:+e}", max_float64);
let min_float64_repr: String = format!("{:+e}", min_float64);

println!("Example of Floats:");
println!("------------------");
println!("{min_float32_repr} <= f32 <= {max_float32_repr}");
println!("{min_float64_repr} <= f64 <= {max_float64_repr}");
```

#### Notes: Numeric Operations

- Supports the basic mathematical operations for all the number types
  - Addition
  - Subtraction
  - Multiplication
  - Division
  - Remainder
- **Integer division truncates toward zero**

```rs
// Examples of Numeric Operations
// ------------------------------
// Addition
let sum: i32 = 5 + 10;
// Subtraction
let difference: f64 = 95.5 - 4.3;
// Multiplication
let product: i32 = 4 * 30;
// Division
let quotient: f64 = 56.7 / 32.2;
let truncated: i32 = -5 / 3; // Truncates toward 0: Results in -1
// Remainder
let remainder_i32: i32 = 43 % 5;
let remainder_f64: f64 = 43.5 % 5.6;

println!("Example of Numeric Operations:");
println!("------------------------------");
println!("5 + 10 = {sum}");
println!("95.5 - 4.3 = {difference}");
println!("4 * 30 = {product}");
println!("56.7 / 32.2 = {quotient}");
println!("-5 / 3 = {truncated}");
println!("43 % 5 = {remainder_i32}");
println!("43.5 % 5.6 = {remainder_f64}");
```

#### Booleans

- 2 possible values: `true` or `false`
- 1 byte in size
- **Specified using `bool`**
- **Booleans are mostly used for *Conditionals***

```rs
// Examples of Booleans
// --------------------
let is_answer: bool = true;
let is_reply: bool = false;

println!("Example of Booleans:");
println!("--------------------");
println!("is_answer = {is_answer}");
println!("is_reply = {is_reply}");
```

#### Characters

- Represent a single character
- Rust's most primitive alphabetic type
- **Represented as a *Unicode Scalar Value***
  - **4-bytes in size**
  - Range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive
  - Represents more than just ASCII characters
- **Specified using `char`**
  - Use single-quotes `''`
  - Double-quotes `""` are for strings
- **NOTE: A *"character"* is not really a concept in Unicode**
  - It is not completely equivalent to an actual `char`
  - Related to *Character Encoding*

```rs
// Examples of Characters
// ----------------------
let small: char = 'z';
let euro: char = '\u{20AC}';
let heart_eyed_cat: char = '😻';

println!("Example of Characters:");
println!("----------------------");
println!("small = {small}");
println!("euro = {euro}");
println!("heart_eyed_cat = {heart_eyed_cat}");
```

### Compound Types

- Group multiple values into one type
- Rust has two primitive compound types
  - *Tuples*
  - *Arrays*

#### Tuples

- Grouping of a number of values with a **variety of types** (heterogeneous) into one type
- **Fixed-length: Once declared, cannot grow or shrink in size**
- **Literal Format: Comma-separated list of values inside parentheses `()`**
  - **Each position in the tuple has a type**
  - **Types of the different values in the tuple do not have to be the same**

```rs
// Example of a Tuple
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

- *A tuple is considered a single compound element*
- To get the values from the tuple, we *unpack*/*destructure* the tuple

```rs
// Example of tuple unpacking
let tup: (i32, f64, i8) = (500, 6.4, 1);
let (x, y, z) = tup;
```

- We can also access a tuple's element directly using `tup.<index>`
- **The first element is index `0`**

```rs
// Example of tuple element access
let equal_x: bool = x == tup.0;
let equal_y: bool = y == tup.1;
let equal_z: bool = z == tup.2;
```

- **NOTE: A tuple without value is called *Unit***
  - Notation: `()`
  - Represent an empty value or an empty return type
  - *Expressions implicitly return the Unit value if they do not return any other value*

```rs
// Example of Tuples
// -----------------
let tup: (i32, f64, i8) = (500, 6.4, 1);
let (x, y, z) = tup;
let equal_x: bool = x == tup.0;
let equal_y: bool = y == tup.1;
let equal_z: bool = z == tup.2;

println!("Example of Tuples:");
println!("------------------");
println!("tup = {tup:?}");
println!("x = {x}");
println!("y = {y}");
println!("z = {z}");
println!("x == tup.0 ? {equal_x}");
println!("x == tup.1 ? {equal_y}");
println!("x == tup.2 ? {equal_z}");
```

#### Arrays

- A collection of **same-type** values (Homogeneous)
- **Every elements in an array must have the same type**
- **Fixed-length: Once declared, cannot grow or shrink in size**
- **Literal Format: Comma-separated list of values in square brackets `[]`**
- Useful for:
  - Allocating data on the *Stack* (instead of *Heap*)
  - To ensure we always have a fixed number of elements
- **The array's type is specified with square brackets `[]`** with:
  - The type of the contained elements
  - A semicolon `;`
  - The number of elements in the array (array-length)

```rs
// Example of an Array
let arr: [i32; 5] = [1, 2, 3, 4, 5];
```

- We can also *initialize* **an array of repeated *same-element*** with just 1 element and the length

```rs
// Example of an Array with same element repeated
let arr_10: [i8; 10] = [5; 10];
```

- **NOTE: A `vector` is the dynamic version of an `array`**
  - Allowed to grow and shrink in size
  - Provided by the standard library
  - Most of the time, a `vector` is what we want to use
  - **Arrays are more useful when the number of elements will not change**

```rs
// Example of an Array
// -------------------
let arr: [i32; 5] = [1, 2, 3, 4, 5];
// Example of an Array with same element repeated
let arr_10 = [5; 10];
// Example of a good use of an array: Elements will not change
const MONTHS: [&str; 12] = [
    "January", "February", "March", "April",
    "May", "June", "July", "August",
    "September", "October", "November", "December"
];

println!("Example of Array:");
println!("-----------------");
println!("arr = {arr:?}");
println!("MONTHS = {MONTHS:#?}");
println!("arr_10 = {arr_10:?}");
```

##### Accessing Array Elements

- Array is a single chunck of fixed-size memory **allocated on the Stack**
- Array elements are accessed via indexing

```rs
// Example of Accessing Array Elements
// -----------------------------------
let some_nums: [i32; 5] = [1, 2, 3, 4, 5];
let first: i32 = some_nums[0];
let second: i32 = some_nums[1];

println!("Example of Accessing Array Elements:");
println!("------------------------------------");
println!("some_nums = {some_nums:?}");
println!("first = {first}");
println!("second = {second}");
```

- **NOTE: Entering index beyond the end of the array result in *Runtime `panic` with `index out of bounds` error***
  - Rust checks for index bounds during runtime
  - In other low-level languages, this check is non-existent
  - Rust ensures proper memory safety principles

## Functions

- Functions are prevalent in Rust
- **`main()` is one of the most important function in Rust**
  - The entry-point of any executable program
- **`fn` allows to declare a new function**
  - Function-names are in `snake_case` format
  - Similar to variables
- **Functions defined in the same file as `main()` can be called directly in `main()`**
  - Rust does not care where the functions are defined
  - As long as they are accessible in the scope of the caller

```rs
fn <func_name>() {
    // Function body defined here
}
```

```rs
fn main() {
    // Example of Function
    // -------------------
    println!("Example of Function:");
    println!("--------------------");
    some_func();
}

/// Example of a Function.
fn some_func() {
    println!("This is printing from some_func().");
}
```

### Parameters

- Functions can have parameters
  - Special variables that are part of a function’s signature
  - When calling the function, we can pass it concrete values as *arguments* to the *parameters*
- **In function declaration, we must declare the type of each parameter**
  - This helps the compiler to be more performant
  - Able to give more helpful error messages

```rs
fn main() {
    // Example of Function With One Parameter
    // --------------------------------------
    println!("Example of Function With One Parameter:");
    println!("---------------------------------------");
    param_func(5);
    println!();

    // Example of Function With Multiple Parameters
    // --------------------------------------------
    println!("Example of Function With Multiple Parameters:");
    println!("---------------------------------------------");
    print_labeled_measurement(5, "cm");
}

/// Example of function with one parameter.
fn param_func(x: i32) {
    println!("The value of param is: {x}");
}

/// Example of function with multiple parameters.
fn print_labeled_measurement(value: i32, unit_label: &str) {
    println!("The measurement is: {value} {unit_label}");
}
```

### Statements and Expressions

- **Function bodies are made up of a series of statements**
  - Optionally ending in an expression
  - Expressions can be part of a statement
- **Rust is an expression-based language**
  - *It is important to understand the difference in Rust*

Term|Definition
:-|:-
**Expressions**|Evaluate and *result in a value*
**Statements**|Instructions that perform some action and *do not return a value*

```rs
// Example of Statements
// ---------------------
println!("Example of Statements:");
println!("----------------------");
let y: i32 = 6; // This is a statement
println!("The value of y is {y}");
```

- **Function-definitions are also statements**
- **Statements do not return values**
  - We cannot assign a `let` statement to another variable
  - There would be nothing for the variable to bind to
- **Statements end with semi-colons**

```rs
// This is an error:
// (let y = 6) is a statement
// It returns no value to bind to x
let x = (let y = 6);
```

- **Expressions always evaluate to a returned value**
  - Most of Rust codes are expressions
  - Expressions can be part of a statement
  - Any math operation is an expression
  - Calling a function/macro is also an expression
  - *A new scope block created with curly-braces `{}` is also an expression*
- **Expressions do not end with semicolons**
  - *An expression with a semicolon is a statement*
  - *Statements do not return a value*

```rs
// Example of Expressions
// ----------------------
println!("Example of Expressions:");
println!("-----------------------");
let exp: i32 = {
    let y: i32 = 3;
    // Expressions do not end with semicolons
    // Else, it is considered a statement and does not return a value
    y + 1
};

println!("The value of exp is {exp}");
```

### Functions With Return Values

- Functions can return values to the code that calls them
- **The *return type* of the function must be declared with `-> <type>`**
  - If it is unspecified, that means the function returns nothing
- **The return value of the function is the value of the final expression in the function body**
  - Or we can return earlier by specifying a `return` and a value
  - But most functions return the last expression implicitly
- The value returned from a function can be used as any other value
  - It is the value of calling the function

```rs
/// Example of function that returns a value.
fn get_thousand() -> i32 {
    1000
}

fn main() {
    // Example of Function That Returns a Value
    // ----------------------------------------
    println!("Example of Function That Returns a Value:");
    println!("-----------------------------------------");
    let res: i32 = get_thousand();
    println!("Value from calling get_thousand() is {res}");
}
```

- **Any expression can be a return-value of a function**
- It can be implicit, or explicitly use `return`

```rs
/// Another Example of function that returns a value.
fn plus_one(x: i32) -> i32 {
    return x + 1;
}

fn main() {
    // Example of Function That Returns a Value
    // ----------------------------------------
    println!("Example of Function That Returns a Value:");
    println!("-----------------------------------------");
    let res2: i32 = plus_one(res);
    println!("Value from calling plus_one(res) is {res2}");
}
```

- **If we change the expression into a statement with `;`, we get an error**
  - Error at compile-time: `mismatched types`
  - Expecting a return value but statement evaluate to `()` (*Unit Type*)
  - Rust often provides messages to possibly help rectify the issue

```rs
/// Example of function that returns nothing.
fn plus_one(x: i32) -> i32 {
    x + 1; // This is a statement: This will throw an error as the return is still exptected to be i32.
}
```

## Comments

- **Comments are ignored by the compiler**
- Mainly helpful for reading codes and for documentation
- There are 3 types of comments in Rust

Type|Description
:-|:-
**Inline Comment**|- Start with `//`<br>- Ignore until the end of the line
**Block Comment**|- Start with `/*`<br>- Ignore until `*/`<br>- Does not nest
**Docstring Comment**|- Used for documenting functions and "objects"<br>- Start with `///`<br>- Same effect as *Inline Comments*<br>- These are picked-up by `rustdoc` and compiled into documentations<br>- Rust codes can be put inside triple-ticks <code>```</code>

```rs
// Inline Comment
// hello, world
```

```rs
/*
So we're doing something complicated here, long enough that we need
multiple lines of comments to do it! Whew! Hopefully, this comment will
explain what’s going on.
*/
```

```rs
/// A human being is represented here.
pub struct Person {
    /// A person must have a name, no matter how much Juliet may hate it.
    name: String,
}

/// Creates a person with the given name.
///
/// # Examples
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
/// use doc::Person;
/// let person = Person::new("name");
/// ```
pub fn new(name: &str) -> Person {
    Person {
        name: name.to_string(),
    }
}
```

## Control Flow

- *Conditioning*: The ability to run some code depending on whether a condition is `true`
- *Looping*: The ability to run some code repeatedly as long as a condition is `true`

### `if-else` Expression

- Allows to branch code depending on conditions
- Blocks of code associated with the conditions are called *arms*
- We can give an optional `else` expression
  - An alternative block of code to execute if the condition evaluates to `false`
  - `else` block is optional: If not given, the execution just moves on

```rs
// Example of If-Else Expression
// -----------------------------
println!("Example of If-Else Expression:");
println!("------------------------------");
let number: i32 = 3;

if number < 5 {
    println!(">> Condition was true");
} else {
    println!(">> Condition was false");
}
```

- ***NOTE: The condition code must evaluate to a boolean***
  - If not, we get a compile-time error
  - **Rust will not automatically try to convert non-Boolean types to a Boolean**
    - Rust does not interpret *Truthy* and *Falsy* values
    - I.e. The value of the expression needs to be a boolean

#### `else if` Expression

- `else if` expressions allow to specify additional conditions
  - Checks each `if` expression in turn
  - Executes the **first** body for which the condition evaluates to `true`
  - ***No cascades*: Exits the forks once a matching path has been determined**
    - Only one result is allowed
    - Whichever comes first that satisfies the condition is used

```rs
// Example of else if Expression
// -----------------------------
println!("Example of else if Expression:");
println!("------------------------------");
let number: i32 = 6;

if number % 4 == 0 {
    println!(">> {number} is divisible by 4");
} else if number % 3 == 0 {
    println!(">> {number} is divisible by 3");
} else if number % 2 == 0 {
    println!(">> {number} is divisible by 2");
} else {
    println!(">> {number} is not divisible by 4, 3, or 2");
}
```

- **NOTE: Too many `if/else if/else` can clutter code**
  - If so, maybe using `match` is a better option

#### Using `if` in `let` Statement

- **`if` is an expression: It returns a value**
  - We can use it on the right-side of `let` variable assignment
  - This is similar to *Conditional Expression* in other languages (e.g. Python)

```rs
// Example of Using if With let
// ----------------------------
println!("Example of Using if With let:");
println!("-----------------------------");
let condition: bool = true;
let number: i32 = if condition { 5000i32 } else { 9000i32 };

println!(">> The value of number is: {number}");
```

- `number` will be bound to a value based on the outcome of the `if` expression
- `{ 5000i32 }` and `{ 6000i32 }` are blocks with expressions `5000` and `6000`
  - The value of the whole `if` expression depends on the block of code that executes
  - **Values that have the potential to be results from each arm of the `if` must be of the same type**
  - In this case, they are both `i32`
  - **If the types do not match, we get an `mismatched type` error**

```rs
// This is an error: `if` and `else` have incompatible types
let number: i32 = if condition { 5i8 } else { 6i32 };
```

- **Variables must have a single type**
  - Rust needs to know *at compile time* what type is assigned to `number`
  - Knowing the type of `number` allows the compiler to verify that the type is valid everywhere we use `number`

### Repetition With Loops

- Rust provides several loop structures
  - `loop`
  - `while`
  - `for`

#### Using `loop`

- Allows to execute a block of code forever or until explicitly told to stop
- **The program will not stop until interupted with `ctrl+c`**
- *This is basically an Infinite Loop, equivalent to a `while true` without warnings*

```rs
// Example of Infinite Loops Using `loop`
// --------------------------------------
println!("Example of Infinite Loops Using `loop`:");
println!("---------------------------------------");
loop {
    println!("run again!");
}
```

- We can break out of the infinite loop using conditions and `break`
- We can also use `continue` to skip an iteration

```rs
// Example of Controlled Loops Using `loop` and `break`
// ----------------------------------------------------
println!("Example of Controlled Loops Using `loop` and `break`:");
println!("-----------------------------------------------------");
let mut i: i32 = 0;
loop {
    if i == 10 {
        break;
    }
    println!("run again! {i}");
    i += 1;
}
println!();
```

#### Returning Values From Loops

- With `loop`, we can retry an operation we know might fail
  - E.g. Checking whether a thread has completed its job
- We might want to capture values out of the loop
  - **We can add the value to return from the loop after the `break` expression**
  - This value will be returned from the loop as the value of the the `loop` expression

```rs
// Example of Returning Values From Loop
// -------------------------------------
println!("Example of Returning Values From Loop");
println!("-------------------------------------");
let mut counter: i32 = 0;

// Capture the returned value from the loop
let result: i32 = loop {
    if counter == 10 {
        // Return the value from the loop
        break counter * 2
    }
    counter += 1;
};

println!("The result from the loop is {result}");
```

- We can also always use `return` to return early
  - `break` only exits and returns from a loop
  - `return` exits and returns from a function call

#### Loop Labels: Disambiguate Between Multiple Loops

- For nested loops, `break` and `continue` apply to the *innermost* loop
- **To apply them to outer loops instead, we use labels to specify the loop to apply to**
- **Loop labels must begin with a single quote `'`**

```rs
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
        if count == 2 {
            // Break from the outer loop
            break 'counting_up;
        }
        println!("\tremaining = {remaining}");
        if remaining == 5 {
            // Break from the inner loop
            break;
        }
        remaining -= 1;
    }

    count += 1;
}
println!("End count = {count}");
```

#### Conditional Loops with `while`

- **While the condition is `true`, the loop will run**
- When the condition ceases to be `true`, the program automatically calls `break`
- We could use a combination of `loop`, `if`, `else`, and `break` to simulate this
- **But Rust has dedicated `while` loop**
  - Eliminates unecessary nesting from using `loop`, `if`, `else`, and `break`

```rs
// Example of while Loop
// ---------------------
println!("Example of while Loop");
println!("---------------------");
let mut number: i32 = 10;

while number != 0 {
    print!("{number}... ");
    number -= 1;
}

println!("LIFTOFF!!!");
```

#### Looping Through a Collection with `for`

- `while` can be used to loop over the elements of a collection

```rs
// Example of while Loop Over Array
// --------------------------------
println!("Example of while Loop Over Array");
println!("--------------------------------");
let arr: [i32; 5] = [10, 20, 30, 40, 50];
let mut index: usize = 0;

while index < 5 {
    println!("The value is: {}", arr[index]);
    index += 1;
}
```

- **However, this approach is error prone and slow**
  - Can cause the program to panic if the index value or test condition is incorrect
  - Compiler adds runtime code to perform the conditional check
- **More concise alternative: Use `for`-loop**
  - Execute some code for each item in a collection
  - Increase the safety of the code
  - Eliminate possible bugs from *overindexing* or *underindexing*
  - Easier to maintain in case the array changes
- **Most Rustaceans would use a `for`-loop over `while`-loop**
  - The safety and conciseness of `for`-loops make them the most commonly used loop construct in Rust

```rs
// Example of Using for Loop Over Array
// ------------------------------------
println!("Example of Using for Loop Over Array");
println!("------------------------------------");
let arr: [i32; 5] = [10, 20, 30, 40, 50];

for el in arr {
    println!("The value is: {el}");
}
```

- **We could also use `Range` from the standard library with `for` loops**
  - Generates all numbers in sequence
  - This is similar to `range` in Python and Go
  - However, the syntax is different
  - *`Stop` is Up-to-but-not-including*
  - `.rev()` allows to *reverse* the range

```rs
// Using Range With for-Loops
// --------------------------
println!("Using Range With for-Loops");
println!("--------------------------");
for num in (1..11).rev() {
    print!("{num}... ");
}
println!("LIFTOFF!!!");
```

## Project Ideas For Practices

- Convert temperatures between Fahrenheit and Celsius
- Generate the $n$th Fibonacci number
- Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song

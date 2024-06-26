# Common Programming Concepts

---

- [Variables and Mutability](#variables-and-mutability)
  - [Constants](#constants)
  - [Shadowing](#shadowing)
- [Data Types](#data-types)
  - [Scalar Types](#scalar-types)
    - [Integers](#integers)
      - [Integer Overflow](#integer-overflow)
    - [Floating-Points](#floating-points)
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

---

## Variables and Mutability

- **By default, variables are immutable**
  - Once a value is bound to a *name*, it cannot be changed
  - **Reassigning to an immutable variable is a compile-time error**
  - Ensures safety and easy concurrency
  - *Mutability can lead to bugs if not managed properly*
  - Cause of bug can be difficult to track down after the fact
  - Immutability makes code easier to reason with

```rs
fn main() {
    // Variable is immutable by default
    let x = 55;
    println!("The value of x is: {x}");
    println!();

    // Reassigning to an immutable variable is a compile-time error
    // This line will generate an error
    // x = 6;
    // error[E0384]: cannot assign twice to immutable variable `x`
}
```

- **We can still change variables to be mutable when needed**
  - **Add `mut` keyword**
  - Explicitly conveys intent that other parts of the code will change this variable
  - Deciding to use mutability is up to you
  - Depends on what you think is clearest in that particular situation

```rs
fn main() {
    // Variable is immutable by default
    // But adding keyword `mut` makes it mutable
    let mut y = 78;
    println!("The value of y is: {y}");

    // This line will not generate an error
    y = 1024;
    println!("Now, the value of y is: {y}");
}
```

### Constants

- Also designed to be bound once to a name and not change
- **Cannot be set `mut`: Constants are *always* immutable**
- Declare using `const` keyword
  - Type *must* be annotated: Constant's types cannot be inferred
  - Can be declared in any scope, including the *global* scope
  - Useful to set globally-fixed values that all parts of an app need to know about
- **Can only be set to a *fixed constant expression*, not results of runtime computation**

```rs
const HOUR_IN_SECONDS: u32 = 60 * 60;
const PI: f64 = 3.14159265359;
```

- **Naming Convention: *Use all-uppercase with underscores between words***
- There is a [limited set of expressions](https://doc.rust-lang.org/reference/const_eval.html) that can be used for constants
  - Can make code easier to understand
  - Gives meaning to the value of the derived constant
- **Constants are valid for the duration of the program runs**
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
    let x = 5;
    println!("The value of x is: {x}");

    // Variable Shadowing is not the same as `mut`
    // Variable Shadowing redeclares the variable with `let`
    // The variable itself does not mutate:
    // We are creating a new variable each time
    let x = x + 6;
    println!("Local-Scope: The value of x is now: {x}");
    {
        // In a different scope, this also shadows the same x
        let x = x * 2;
        println!("Inside-Scope: The value of x in the inner scope is: {x}");
    }
    // After the scope ends, the shadowing also ends
    // This one is back to the previous shadow
    println!("Local-Scope: The value of x is back to: {x}");
}
```

- **Shadowing is different from marking a variable as `mut`**
  - Compile-time error if we accidentally try to reassign to this variable without using the `let` keyword
  - **The variable itself does not mutate: We are creating a new variable each time**
  - We can apply some transformation on the *value* without affecting the *variable*
- **We can also change the type of the variable while shadowing the same name**
  - Because we are essentially creating a new variable anyway
  - Shadowing allows to reuse the name without a need to create a new variable name
  - However, the old value is gone (unless in a different scope)

```rs
let spaces = "   ";         // String type
let spaces = spaces.len();  // Number type
```

- Using `mut` for this will get us a compile error
  - **We are not allowed to mutate a variable's type**

```rs
let mut spaces = "   "; // String type
// The following will generate an error: Cannot mutate a variable's type
// spaces = spaces.len();  // Number type
```

## Data Types

- Rust is a statically-typed language
  - **Every value in Rust is of a specific data type**
  - **All types of all variables must be known at compile time**
- Compiler can infer the type based on the value
  - *When many types are possible (E.g. integers), we must add a type annotation*

```rs
// Explicit type
let guess: u32 = "42".parse().expect("Not a number!");
```

- There are 2 categories of data types
  - *Scalar*
  - *Compound*

### Scalar Types

- Represent single values
- There are 4 primary scalar types
  - *Integers*
  - *Floating-Points*
  - *Booleans*
  - *Characters*

#### Integers

- Number without a fractional component
- Rust has multiple *Integer* types
  - *Signed Integers* start with `i`
  - *Unsigned Integers* start with `u`
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
  - `isize` and `usize` are useful when indexing some sort of collection
- Number literals that can be multiple numeric types allow a type suffix to specify
  - `57u8`
  - `57i32`
- Number literals can use `_` as 1000 separators

Supported Integer literals|Example
:-|:-
Decimal|`98222`, `98_222`
Hex|`0xff`
Octal|`0o77`
Binary|`0b11110000`, `0b1111_0000`
Byte (`u8` only)|`b'A'`

```rs
// Examples of Signed Integers
let byte: i8 = -128;
let short: i16 = -32_768;
let int: i32 = -2_147_483_648;
let long: i64 = -9_223_372_036_854_775_808;
let llong: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;

// Examples of Unsigned Integers
let ubyte: u8 = 255;
let ushort: u16 = 65_535;
let uint: u32 = 4_294_967_295;
let ulong: u64 = 18_446_744_073_709_551_615;
let ullong: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
```

##### Integer Overflow

- ***Integer Overflow* is when we assign a value that is larger than the max of the integer type**
- Results in one of 2 behaviors:
  1. In *Debug* mode, checks for integer overflow will cause `panic` (error) at runtime
  2. In *Release* mode, no panic but performs two’s complement wrapping instead: *wrap around* to the minimum of the type
- ***NOTE: Relying on integer overflow’s wrap-around behavior is considered an error***
  - **All possible overlfow should be handled explicitly**

Handling Approach|Description
:-|:-
`wrapping_*` Methods|Wrap in all modes. E.g `wrapping_add()`
`checked_*` Methods|Return the `None` value if there is overflow
`overflowing_*` Methods|Return the value and a boolean indicating whether there was overflow
`saturating_*` Methods|Saturate at the value’s minimum or maximum values

#### Floating-Points

- 2 primitive types for float-values
- **Default Float Type: `f64`**
  - On modern CPUs, roughly the same speed as `f32`
  - But is capable of more precision
- **All Floating-Points are signed**
- **Represented according to the IEEE-754 standard**

Length|Type
:-|:-
32-bit|`f32`
64-bit|`f64`

```rs
// Examples of Floating-Points
let max_float32: f32 = 3.4028235e38;
let min_float32: f32 = -3.4028235e38;
let max_float64: f64 = 1.7976931348623157e308;
let min_float64: f64 = -1.7976931348623157e308;
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
// Addition
let sum = 5 + 10;
// Subtraction
let difference = 95.5 - 4.3;
// Multiplication
let product = 4 * 30;
// Division
let quotient = 56.7 / 32.2;
let truncated = -5 / 3; // Results in -1
// Remainder
let remainder_i32 = 43 % 5;
let remainder_f64 = 43.5 % 5.6;
```

#### Booleans

- `true` or `false`
- 1 byte in size
- **Specified using `bool`**

```rs
// Examples of Booleans
let answer: bool = true;
let reply: bool = false;
```

- **Booleans are mostly used for *Conditionals***

#### Characters

- Represent a single character
- Rust's most primitive alphabetic type
- Represented as a *Unicode Scalar Value*
  - **4-bytes in size**
  - Range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive
- **Specified using `char`**
  - Use single-quotes `''` (Double-quotes `""` are for strings)
- **NOTE: A character is not really a concept in Unicode**
  - Is not completely equivalent to an actual `char`
  - Related to *Character Encoding*

```rs
// Example of Characters
let c = 'z';
let z: char = 'ℤ';
let heart_eyed_cat: char = '😻';
```

### Compound Types

- Group multiple values into one type
- Rust has two primitive compound types
  - *Tuples*
  - *Arrays*

#### Tuples

- Grouping of a number of values with a variety of types into one type
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
let (x: i32, y: f64, z: i8) = tup;
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

#### Arrays

- A collection of **same-type** values
- **Every elements in an array must have the same type**
- **Fixed-length: Once declared, cannot grow or shrink in size**
- **Literal Format: Comma-separated list of values square brackets `[]`**
- Useful for:
  - Allocating data on the Stack (instead of Heap)
  - To ensure we always have a fixed number of elements
- The array's type is specified with square brackets `[]` with:
  - The type of the contained elements
  - A semicolon
  - The number of elements in the array (array-length)

```rs
// Example of an Array
let arr: [i32; 5] = [1, 2, 3, 4, 5];
```

- We can also *initialize* and array of repeated *same-element* with just 1 element and the length

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
// Example of when to use an Array
const MONTHS: [&str; 12] = [
    "January", "February", "March",
    "April", "May", "June",
    "July", "August", "September",
    "October", "November", "December"
];
```

##### Accessing Array Elements

- Array is a single chunck of fixed-size memory allocated on the stack
- Array elements are accessed via indexing

```rs
// Example of Accessing Array Elements
let some_nums = [1, 2, 3, 4, 5];
let first = some_nums[0];
let second = some_nums[1];
```

- **NOTE: Entering index beyond the end of the array result in *Runtime `panic` with `index out of bounds` error***
  - Rust checks for index bounds during runtime
  - In other low-level languages, this check is non-existent
  - Rust ensures proper memory safety principles

## Functions

- Functions are prevalent in Rust
- `main()` is one of the most important function in Rust
  - The entry-point of any executable program
- **`fn` allows to declare a new function**
- Functions are in `snake_case` format
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
    println!("Example of Function:");
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
    println!("Example of Function With Parameter:");
    param_func(5);
    print_labeled_measurement(5, 'h');
}

fn param_func(x: i32) {
    println!("The value of param is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

### Statements and Expressions

- Function bodies are made up of a series of statements
  - Optionally ending in an expression
  - Expressions can be part of a statement
- **Rust is an expression-based language**
  - *It is important to understand the difference in Rust*

Term|Definition
:-|:-
**Expressions**|Evaluate and *result in a value*
**Statements**|Instructions that perform some action and *do not return a value*

```rs
// Example of statements
let y = 6;
println!("The value of y is {y}");
```

- Function-definitions are also statements
- **Statements do not return values**
  - We cannot assign a `let` statement to another variable
  - There would be nothing for the variable to bind to
  - **Statements end with semi-colons**

```rs
// This is an error: (let y = 6) returns no value to bind to x
let x = (let y = 6);
```

- **Expressions always evaluate to a value**
  - Most of Rust codes are expressions
  - Expressions can be part of a statement
  - Any math operation is an expression
  - Calling a function/macro is also an expression
  - A new scope block created with curly brackets is also an expression
- **Expressions do not end with semicolons**
  - *An expression with a semicolon is a statement*
  - *Statements do not return a value*

```rs
// Example of expressions
let exp = {
    let y = 3;
    y + 1
};

println!("The value of exp is: {exp}");
```

### Functions With Return Values

- Functions can return values to the code that calls them
- **The *return type* of the function must be declared with `-> <type>`**
- **The return value of the function is the value of the final expression in the function body**
  - Or we can return earlier by specifying a `return` and a value
  - But most functions return the last expression implicitly
- The value returned from a function can be used as any other value

```rs
// Example of function that return a value.
fn get_thousand() -> i32 {
    1000
}

fn main() {
    // Calling a function with return.
    let res = get_thousand();
    println!("Value from calling get_thousand() is {res}");
}
```

- **Any expression can be a return-value of a function**

```rs
// Example of function that return a value.
fn plus_one(x: i32) -> i32 {
    x + 1
}
```

- **If we change the expression into a statement with `;`, we get an error**
  - Error at compile-time: `mismatched types`
  - Expecting a return value but statement evaluate to `()` (*Unit Type*)
  - Rust often provides messages to possibly help rectify the issue

```rs
// Example of function that return nothing.
// This will throw an error as the return is still exptected to be i32.
fn plus_one(x: i32) -> i32 {
    x + 1; // This is a statement
}
```

### Comments

- Comments are ignored by the compiler
- Mainly helpful for reading codes and for documentation
- There are 3 types of comments in Rust
  - **Inline Comment**
    - Start with `//`
    - Ignore until the end of the line
  - **Block Comment**
    - Start with `/*`
    - Ignore until `*/`
    - Does not nest
  - **Doc Comment**
    - Used for documenting functions and "objects"
    - Start with `///`
    - Same effect as *Inline Comments*
    - These are picked-up by `rustdoc` and compiled into documentations

```rs
// Inline Comment
// hello, world

/*
So we’re doing something complicated here, long enough that we need
multiple lines of comments to do it! Whew! Hopefully, this comment will
explain what’s going on.
*/

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

- The ability to run some code depending on whether a condition is `true`
- The ability to run some code repeatedly as long as a condition is `true`

### `if-else` Expression

- Allows to branch code depending on conditions
- Blocks of code associated with the conditions are called *arms*
- We can give an optional `else` expression
  - An alternative block of code to execute if the condition evaluate to `false`
  - `else` block is optional: If not given, the execution just move on

```rs
// Example of If-Else Expression
// -----------------------------
let number = 3;

if number < 5 {
    println!(">> Condition was true");
} else {
    println!(">> Condition was false");
}
```

- ***NOTE: The condition code must be a bool***
  - If not, we get a compile-time error
  - I.e. The value of the expression needs to be a boolean
  - **Rust will not automatically try to convert non-Boolean types to a Boolean**

#### `else if` Expression

- `else if` expressions allow to specify additional conditions
  - Checks each `if` expression in turn
  - Executes the **first** body for which the condition evaluates to `true`
  - ***No cascades*: Exits the forks once a path is determined**
    - Only one result is allowed
    - Whichever comes first that satisfies the condition is used

```rs
// Example of else if Expression
// -----------------------------
let number = 6;

if number % 4 == 0 {
    println!(">> number is divisible by 4");
} else if number % 3 == 0 {
    println!(">> number is divisible by 3");
} else if number % 2 == 0 {
    println!(">> number is divisible by 2");
} else {
    println!(">> number is not divisible by 4, 3, or 2");
}
```

- **NOTE: Too many `if/else if/else` can clutter code**
  - If so, maybe using `match` is a better option

#### Using `if` in `let` Statement

- **`if` is an expression**
- We can use it on the right-side of `let`
- This looks like a *Conditional Expression* in other languages (e.g. Python)

```rs
// Example of Using if With let
// ----------------------------
let condition = true;
let number = if condition { 5 } else { 6 };

println!("The value of number is: {number}");
```

- `number` will be bound to a value based on the outcome of the `if` expression
- `{ 5 }` and `{ 6 }` are blocks with expressions `5` and `6`
  - The value of the whole `if` expression depends on which block of code executes
  - **Values that have the potential to be results from each arm of the `if` must be the same type**
  - In this case, they are both `i32`
  - If the types do not match, we get an error

```rs
// This is an error: `if` and `else` have incompatible types
let number = if condition { 5 } else { "six" };
```

- Variables must have a single type
- Rust needs to know at compile time what type `number` is
- Knowing the type of `number` lets the compiler verify the type is valid everywhere we use `number`

### Repetition With Loops

- Rust provides several loop structures
  - `loop`
  - `while`
  - `for`

#### Using `loop`

- Allows to execute a block of code forever or until explicitly told to stop
- **The program will not stop until interupted with `ctrl+c`**
- *This is basically an Infinite Loop*

```rs
// Example of Infinite Loops Using `loop`
// --------------------------------------
loop {
    println!("again!");
}
```

- We can break out of the infinite loop using `break`
- We can also use `continue` to skip an iteration

#### Returning Values From Loops

- With `loop`, we can retry an operation we know might fail
  - E.g. Checking whether a thread has completed its job
- We might want to capture values out of the loop
- **We can add the value to return from the loop after the `break` expression**
- This value will be returned from the loop

```rs
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
```

#### Loop Labels: Disambiguate Between Multiple Loops

- For nested loops, `break` and `continue` apply to the innermost loop
- **To apply them to outer loops instead, we use labels to specify the loop**
- **Loop labels must begin with a single quote `'`**

```rs
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
```

#### Conditional Loops with `while`

- **While the condition is `true`, the loop will run**
- When the condition ceases to be `true`, the program calls `break`
- We could use a combination of `loop`, `if`, `else`, and `break`
- **But Rust has `while` loop**
  - Eliminates unecessary nesting from using `loop`, `if`, `else`, and `break`
  -

```rs
// Example of while loop
// ---------------------
let mut number = 3;

while number != 0 {
    println!("{number}!");
    number -= 1;
}

println!("LIFTOFF!!!");
```

#### Looping Through a Collection with `for`

- `while` can be used to loop over the elements of a collection

```rs
// Example of while loop Over Array
// --------------------------------
let a = [10, 20, 30, 40, 50];
let mut index = 0;

while index < 5 {
    println!("the value is: {}", a[index]);
    index += 1;
}
```

- However, this approach is error prone
  - Can cause the program to panic if the index value or test condition is incorrect
  - Also slow: Compiler adds runtime code to perform the conditional check
- More concise alterntive: Use `for` loop
  - Execute some code for each item in a collection
  - Increase the safety of the code
  - Eliminated possible bugs from overindexing or underindexing
  - Easier to maintain in case the array changes
- Most Rustaceans would use a `for` loop over `while` loop

```rs
// Example of for loop Over Array
// ------------------------------
let a = [10, 20, 30, 40, 50];
for el in a {
    println!("The value is: {el}");
}
```

- We could also use `Range` from the standard library with `for`
  - Generates all numbers in sequence

```rs
// Using Range With for-loops
// --------------------------
for num in (1..4).rev() {
    println!("{num}!");
}
println!("LIFTOFF!!!");
```

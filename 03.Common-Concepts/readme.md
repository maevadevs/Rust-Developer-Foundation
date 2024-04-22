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
  - The type of each element
  - A semicolon
  - The number of elements in the array (length)

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
  - In other low-level languages, this checks is non-existent
  - Rust ensure proper memory safety principles

## Functions

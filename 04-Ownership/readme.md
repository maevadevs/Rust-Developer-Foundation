# Ownership

---

- [What Is Ownership](#what-is-ownership)
  - [Stack and Heap](#stack-and-heap)
    - [Stack](#stack)
    - [Heap](#heap)
    - [Stack vs Heap](#stack-vs-heap)
  - [Ownerhsip Rules](#ownerhsip-rules)
  - [Variable Scope](#variable-scope)

---

- Rust’s most unique feature
- Has deep implications for the rest of the language
- Allows to make memory safety guarantees without needing a GC
- Other related features
  - Borrowing
  - Slices
  - How Rust lays data out in memory

## What Is Ownership

- **A set of rules that govern how a Rust program manages memory**
  - Some languages use GC to clear memory during runtime
  - Other languages must have their memory managed explicitly
- Rust uses a different approach
  - **Memory is managed through a system of ownership**
  - **There are a set of rules that the compiler checks**
  - **If any of the rules are violated, the program will not compile**
  - *None of the features of ownership will slow down the program while it is running*
- Understanding *Ownership* gives a solid foundation for understanding the features that make Rust unique

### Stack and Heap

- In system programming language, understanding Stack and Heap is essential
  - Whether a value is on the Stack or the Heap affects how the language behaves
  - Also affects why you have to make certain decisions
  - Parts of Ownership is described in relation to the Stack and the Heap
- Stack and the Heap are parts of memory available during runtime
  - But they are structured in different ways

#### Stack

- Last-In, Firt-Out (LIFO) data structure
- Stores values in the order it gets them
- Removes the values in the opposite order
- Adding/Removing from the middle would not work well
- Adding data == Pushing onto the Stack
- Removing data == Popping from the Stack
- **All data stored on the Stack must have a known and fixed-size**
  - Else, store the data in the Heap instead

#### Heap

- Less organized than the Stack
- The memory allocator finds an empty spot in the heap that is big enough
  - Marked as *being used* and store the value there
  - Return a *pointer/address* to that location
  - This process is called *Allocating* the Heap
  - *NOTE: Pushing value unto the Stack is not considered Allocating*
- **The pointer to the Heap is a known and fixed-size value**
  - *This pointer is stored on the Stack*
  - When we want the actual data, we must follow the pointer

#### Stack vs Heap

- **Pushing unto the Stack is faster than allocating unto the Heap**
  - The allocator never has to search for a place to store new data in the Stack
  - Location is always at the top of the Stack
  - For Heap, the allocator must find a big enough space to hold the data
  - Then perform bookkeeping to prepare for the next allocation
- **Accessing data in the Heap is slower than in the Stack**
  - Have to follow a pointer to get to the value
  - Processing is faster if we don't jump around (Stack)
  - A processor is better if it works on data that is close to other data
- **When calling a function, the arguments and parameters get pushed onto the Stack**
  - When the function is over, those values get popped off the Stack
- ***Main purpose of Ownership is to manage Heap Data***
  - Keep track of what parts of code are using what data on the Heap
  - Minimize amount of duplicate data on the Heap
  - Clean up unused data on the Heap

### Ownerhsip Rules

- **Each value in Rust has an owner**
- **There can only be one owner at a time**
- **When the owner goes out of scope, the value will be dropped**

### Variable Scope

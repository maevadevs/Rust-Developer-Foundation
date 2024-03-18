# Introduction

---

- [Installation](#installation)
  - [Installation For Linux/MacOs](#installation-for-linuxmacos)
  - [Installation For Windows](#installation-for-windows)
  - [Installation Profiles](#installation-profiles)
- [Post-Installation Checks](#post-installation-checks)
  - [Check Installed Compiler Version](#check-installed-compiler-version)
  - [Check Local Documentation](#check-local-documentation)
- [Update](#update)
- [Uninstallation](#uninstallation)
- [Hello World](#hello-world)
  - [Compiling Rust Code](#compiling-rust-code)
- [Hello Cargo](#hello-cargo)
  - [Main Commands](#main-commands)
  - [Check `cargo` Version](#check-cargo-version)
  - [Creating A Project](#creating-a-project)
  - [Building A Project](#building-a-project)
    - [Build And Run At Once](#build-and-run-at-once)
    - [Check Pre-Build](#check-pre-build)
    - [Building For Release](#building-for-release)
  - [Cargo As Convention](#cargo-as-convention)

---

## Installation

- **Install via `rustup`**
  - Command-line tool for managing Rust versions
  - Installs both *Rust*, *Cargo*, and additional tools
- `rustup`
  - Home Directory `~/.rustup`
  - Can be modified via `RUSTUP_HOME` environment variable
- `cargo`
  - Home Directory `~/.cargo`
  - Can be modified via `CARGO_HOME` environment variable
- Paths added to
  - `~/.profile`
  - `~/.bash_profile`
  - `~/.bashrc`
  - Environment Variables

### Installation For Linux/MacOs

- Need a *C-Compiler/Linker*
  - Used to join compiled outputs into one file
- For Linux: `gcc` or `clang`

```sh
# GCC
# ---
# Check if already installed
gcc --version
# If not, install with build-essential
sudo apt-get install build-essential

# Clang
# -----
# Check if already installed
clang --version
# If not, search available versions
apt-cache search clang
# Install latest available: In this case, v15
sudo apt-get install clang-15 --install-suggests
```

- For MacOS: Install XCode's compiler

```sh
xcode-select --install
```

- Check if rustup is installed

```sh
# Check if rustup is installed
rustup --version
```

- If not, install `rustup` via `curl`

```sh
# Install rustup via curl
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Installation For Windows

- Need a *C-Compiler/Linker*
  - Used to join compiled outputs into one file
  - Need to install *MSVC Build Tools For Visual Studio 2013 or later* via *Visual Studio Installer*
    - Install Visual Studio
    - Choose *Desktop Development with C++*
    - Choose *Windows 10 or Windows 11 SDK*
- Check if rustup is installed

```sh
# Check if rustup is installed
rustup --version
```

- If not, download and run `rustup-init.exe` from [rust-lang.org](rust-lang.org/tools/install)

### Installation Profiles

Installation Profile|Included
:-|:-
*Minimal*|`rustc`,`rust-std`,`cargo`
*Default*|*Minimal* + `rust-docs`, `rustfmt`, `clippy`
*Complete*|All components available through `rustup`:<br>- Every component ever included in the metadata<br>- *This option should never be used except for maintainers maybe*

## Post-Installation Checks

```sh
# Check the installed toolchain
rustup toolchain list

# List the installed components
rustup component list --installed
```

Component|Description
:-|:-
`cargo`|Rust's package manager and build tool
`clippy`|A collection of lints to catch common mistakes
`rust-docs`|Rust Docs: Access via `rustup docs`
`rust-std`|Rust Standard Library
`rustc`|Rust Compiler
`rustfmt`|Rust Formatter to style guidelines

- A copy of multiple Rust docs are also available via `rustup docs`

Command|Docs For
:-|:-
`rustup docs`|Rust Documentation Index
`--alloc`|The Rust core allocation and collections library
`--book`|*The Rust Programming Book*
`--cargo`|The Cargo Book
`--core`|The Rust Core Library
`--edition-guide`|The Rust Edition Guide
`--embedded-book`|The Embedded Rust Book
`--help`|Prints help information
`--nomicon`|The Dark Arts of Advanced and Unsafe Rust Programming
`--path`|Only print the path to the documentation
`--proc_macro`|A support library for macro authors when defining new macros
`--reference`|The Rust Reference
`--rust-by-example`|A collection of runnable examples that illustrate various Rust concepts and standard libraries
`--rustc`|The compiler for the Rust programming language
`--rustdoc`|Generate documentation for Rust projects
`--std`|Standard library API documentation
`--test`|Support code for rustc's built in unit-test and micro-benchmarking framework
`--unstable-book`|The Rust Unstable Book
`--toolchain <toolchain>`|Toolchain name

### Check Installed Compiler Version

```sh
rustc --version
```

### Check Local Documentation

```sh
rustup doc
```

## Update

```sh
# Updatee all installed rustup components
rustup update
```

## Uninstallation

```sh
rustup self uninstall
```

## Hello World

- Create a `hello-world` project folder
- Add a `src` folder
- Create a `main.rs` file in `src`
- Add the following codes to `main.rs`

```rs
/**
 * Print "Hello, world!" to the console.
 */

fn main() {
    println!("Hello, world!");
}

// ON LINUX:
//  Compile $> rustc src/main.rs -o target/main
//  Execute $> ./target/main
// ON WINDOWS:
//  Compile $> rustc src\main.rs -o target\main.exe
//  Execute $> .\target\main.exe

```

- **`main()` is the entrance in every executable Rust program**
  - No parameters
  - No return values
- Rust requires curly brackets around all function bodies
  - Good style to place the opening curly bracket on the same line as the function declaration
  - Automatic formatter tool: `rustfmt` use with `cargo`
  - **Rust style is to indent with 4 spaces, not a tab**
- **`identifier!()` is a macro**
  - Functions and Macros are different in Rust
  - Macros don’t always follow the same rules as Functions
  - `println!()` is a macro / `println()` is a function
  - Most lines of Rust code end with a semicolon

### Compiling Rust Code

- Rust is *Ahead-Of-Time (AOT)*-compiled language
- Compiler: `rustc`
- On Windows, also outputs debugging info as `.pdb` file
- **`rustc` is not enough for larger programs**
  - **We use `cargo` instead**

## Hello Cargo

- Build System and Package Manager
- Allows to manage projects
  - Build codes
  - Download dependencies
  - Building libraries
- It is better to start a project using `cargo`
  - The vast majority of Rust projects are built with `cargo`
- Cargo comes with Rust when installing via `rustup`
- Full documentation for `cargo` is available in the [Cargo Book](https://doc.rust-lang.org/cargo/)

### Main Commands

The commands are the same no matter which operating system

- `cargo new`
- `cargo build`
- `cargo run`
- `cargo check`

### Check `cargo` Version

```sh
cargo --version
```

### Creating A Project

```sh
cargo new hello-cargo
cd hello-cargo
```

- Creates a new directory and project: `hello-cargo`

```tree
hello-cargo/
|- .gitignore
|- Cargo.toml
|- src/
   |- main.rs
```

Folder or File|Description
:-|:-
`Cargo.toml`|Manage project configs and deps/crates
`src/main.rs`|The entrance of the program
`.git`|Default VCS: Can override with `--vcs` flag
`.gitignore`|Default VCS is `git`: Can override with `--vcs` flag

- **NOTE: Git files are not generated if already within an existing Git repository**
- `Cargo.toml` has multiple sections but the following are defaulted
  - `package`
    - Configure the project as package and compiler info
    - `edition` is the edition of Rust to use
  - `dependencies`
    - List of crates dependencies for the project
    - If none is listed, then the project will install no additional crates
- **All source codes should live in `src`**
- Except for non-source code files, which can be in top-level project directory:
  - README files
  - License information
  - Configuration files
  - Anything else not related to code
- Cargo helps organize projects
  - There is a place for everything
  - Everything is in its place
- ***It is possible to convert a manually-managed project to Cargo project***
  - Move the project code into the `src` directory
  - Create an appropriate `Cargo.toml` file

### Building A Project

- Simply `cd` into the project and run `cargo build`

```sh
cd hello-cargo
cargo build
```

- It builds from `src` folder
  - Creates an executable file in `target/debug`
  - Default build is *Debug Build* (`dev [unoptimized + debuginfo]`)
- First-time run also creates `Cargo.lock`
  - Keeps track of the exact versions of deps/crates in the project
  - *Managed by Cargo: Never need to change this file manually*
  - This file is automatically generated by Cargo
  - It is not intended for manual editing

#### Build And Run At Once

- We can run `cargo run` to build and run at once
- Convenience for *`cargo build` && run binaries*

```sh
cd hello-cargo
cargo run
```

- **NOTE: Cargo will only rebuild if there are changes detected**

#### Check Pre-Build

- `cargo check` allows to check if a project can build without generating the binaries
- Much faster than `cargo build`
  - Skips the step of producing an executable
  - Still generates `target` but not the executable file
  - Helpful for multiple periodical compiling-checks when writing code (e.g. watching for changes, debugging errors)

```sh
cd hello-cargo
cargo check
```

#### Building For Release

- Default build is *Debug Build* (`dev [unoptimized + debuginfo]`)
- To build for release, we use the `--release` option

```sh
cd hello-cargo
cargo build --release
```

- Compiles with optimizations
- Creates an executable file in `target/release`
- Makes Rust code run faster
- But longer build-time is needed
- Only use this for building the final program

### Cargo As Convention

- Cargo is invaluable for large and complex projects
- Additional tooling for project management
- We can use `cargo build` on any project made with `cargo`

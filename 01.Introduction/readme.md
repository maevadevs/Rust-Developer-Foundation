# Introduction

---

- [Installation](#installation)
  - [`rustup`](#rustup)
  - [`cargo`](#cargo)
  - [C-Compiler](#c-compiler)
  - [Installation For Linux/MacOS](#installation-for-linuxmacos)
  - [Installation For Windows](#installation-for-windows)
  - [Installation Profiles](#installation-profiles)
- [Post-Installation Checks](#post-installation-checks)
  - [`rustup docs`](#rustup-docs)
- [Additional `rustup` Commands](#additional-rustup-commands)
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

- Using **`rustup`**
  - Command-line tool for managing Rust versions
  - Installs *Rust*, *Cargo*, and additional developer tools

### `rustup`

Settings|Value|Default Value
:-|:-|:-
Home Directory|`~/.rustup`|
Environment Variable|`RUSTUP_HOME`|`~/.rustup`

- `RUSTUP_HOME` can be used to customize installation

### `cargo`

Settings|Value|Default Value
:-|:-|:-
Home Directory|`~/.cargo`|
Environment Variable|`CARGO_HOME`|`~/.cargo`

- `CARGO_HOME` can be used to customize installation
- Paths are added to:
  - `~/.profile`
  - `~/.bash_profile`
  - `~/.bashrc`
  - Environment Variables

### C-Compiler

- Need a *C-Compiler & Linker*
- Used to join compiled outputs into one file

### Installation For Linux/MacOS

- For Linux, install `gcc` or `clang`
- `build-essential` => `dpkg-dev`, `g++`, `gcc`, `libc6-dev`, `make`

```sh
# GCC
# ---
# Check if already installed
gcc --version

# If not, install via build-essential
sudo apt-get install build-essential
```

```sh
# Clang
# -----
# Check if already installed
clang --version

# If not, search available versions
apt-cache search clang

# Install latest version available: In this case, v15
sudo apt-get install clang-15 --install-suggests
```

- For MacOS, install XCode's compiler

```sh
xcode-select --install
```

- Check if `rustup` is already installed

```sh
# Check if rustup is already installed
rustup --version
```

- If not, install `rustup` via `curl`
- This also bootstraps the installation of the Rust dev tools via `rustup`

```sh
# Install rustup via curl, then Rust dev tools
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Installation For Windows

- For Windows, need to install *MSVC Build Tools For Visual Studio 2013 or later*
- It can be installed with *Visual Studio Installer*
  - Install Visual Studio
  - Choose *Desktop Development with C++*
  - Choose appropriate *Windows SDK*
  - To check the version of the compiler, call with no argument:
    - **x64**: `C:\"Program Files"\"Microsoft Visual Studio"\2022\Community\VC\Tools\MSVC\<<version>>\bin\Hostx64\x64\cl.exe`
    - **x86**: `C:\"Program Files"\"Microsoft Visual Studio"\2022\Community\VC\Tools\MSVC\<<version>>\bin\Hostx64\x86\cl.exe`
- Check if `rustup` is installed

```sh
# Check if rustup is already installed
rustup --version
```

- If not, download and run `rustup-init.exe` from [rust-lang.org](https://rust-lang.org/tools/install)

### Installation Profiles

Installation Profile|Included
:-|:-
**Minimal**|`rustc`,`rust-std`,`cargo`
**Default** (Recommended)|*Minimal* + `rust-docs`, `rustfmt`, `clippy`
**Complete**|*Default* + All components available through `rustup`:<br>Every component ever included in the metadata<br>*This option should never be used except for maintainers*

## Post-Installation Checks

```sh
# Check the installed toolchain
rustup toolchain list

# List the installed components
rustup component list --installed
```

Component|Description
:-|:-
`cargo`|Package Manager / Build Tool
`clippy`|A collection of lints to catch common mistakes
`rust-docs`|All Rust Documentations installed locally for offline use:<br>Access via `rustup docs` or `rustup doc` command
`rust-std`|Rust Standard Library
`rustc`|Rust Compiler
`rustfmt`|Rust Formatter to follow standard style guidelines

### `rustup docs`

- Allows to access Rust Documentations offline
- There are multiple options available

Command|Docs For
:-|:-
`rustup docs`|Rust Documentation Index
`--alloc`|The Rust Core Allocation and Collections library
`--book`|The Rust Programming Book (*The Book*)
`--cargo`|The Cargo Book
`--core`|The Rust Core Library
`--edition-guide`|The Rust Edition Guide
`--embedded-book`|The Embedded Rust Book
`--help`|Prints help about `rustup docs` commands
`--nomicon`|The Rustonomicon: The Dark Arts of Advanced and Unsafe Rust Programming
`--path`|Print the path to the local offline documentations
`--proc_macro`|A support library for macro authors when defining new macros
`--reference`|The Rust Reference
`--rust-by-example`|A collection of runnable examples that illustrate various Rust concepts and standard libraries
`--rustc`|The Rust Compiler Book
`--rustdoc`|The `rustdoc` Book: Documentation Generation tool for Rust projects
`--std`|Standard library API documentation
`--test`|Library to support code for `rustc`'s built-in unit-test and micro-benchmarking framework
`--unstable-book`|The Rust Unstable Book
`--toolchain <toolchain>`|Rust documentations specific to a Toolchain (The toolchain must be installed)

## Additional `rustup` Commands

Command|Description
:-|:-
`rustc --version`|Check installed Rust Compiler version
`rustup docs` or `rustup doc`|Open local documentations index
`rustup update`|Update all installed `rustup` components
`rustup self uninstall`|Uninstall Rust and all its components

## Hello World

- Create a `hello-world` project folder with the following structure

```tree
hello-world/
├── target/
└── src/
    └── main.rs
```

- Open `main.rs` in an editor and add the following codes to `main.rs`

```rs
/****************************************/
/* Print "Hello world!" to the console. */
/****************************************/

/// The entry-point of the program.
fn main() {
    println!();
    println!("Hello world!");
    println!();
}

// ON LINUX:
//      Compile: rustc src/main.rs -o target/main
//      Execute: ./target/main
// ON WINDOWS:
//      Compile: rustc src\main.rs -o target\main.exe
//      Execute: .\target\main.exe
```

- **`main()` function is the entry-point in every *executable* Rust program**
  - No parameters
  - No return values
- **Rust requires curly brackets around all function bodies**
  - Recommended style to place the opening curly bracket on the same line as the function declaration
  - Automatic formatter tool `rustfmt` is used with `cargo`
  - **Rust style is to indent with 4 spaces, not tabs**
- **`macroname!()` is a macro**
  - Functions and Macros are different in Rust
  - Macros do not always follow the same rules as Functions
  - E.g. `println!()` is a macro
- **Most lines in Rust end with a semicolon (`;`), but not all of them (see expression vs statement)**

### Compiling Rust Code

- **Rust is *Ahead-Of-Time (AOT)* compiled language**
  - Produces a single executable file
  - Similar to C, C++, Go
  - Compiler: `rustc`
- On Windows, compiling also outputs a `.pdb` file for debugging info

```sh
# Compiling on Linux
rustc src/main.rs -o target/main

# Executing on Linux
./target/main
```

```ps1
# Compiling on Windows
rustc src\main.rs -o target\main.exe

# Executing on Windows
.\target\main.exe
```

- **NOTE: `rustc` alone is not enough for larger projects**
  - **We use `cargo` instead**
  - `cargo` allows to better manage a larger project

## Hello Cargo

- **Cargo is the *Build System* and *Package Manager* for Rust**
- Allows to manage projects
  - Build codes
  - Download dependencies
  - Build libraries
- **It is better to start a project using `cargo`**
  - The vast majority of Rust projects are built with `cargo`
- Cargo comes with Rust when installing via `rustup`
- Full documentation for `cargo` is available in the [Cargo Book Online](https://doc.rust-lang.org/cargo/) or Offline

```sh
rustup docs --cargo
```

### Main Commands

- The commands are the same no matter which operating system

Command|Description
:-|:-
`cargo new`|Create a new project
`cargo build`|Compile the source codes in the current project (debug mode)
`cargo build --release`|Compile the source codes with optimizations (release mode)
`cargo run`|Build + Run the project (debug mode)
`cargo run --release`|Build + Run the project (release mode)
`cargo check`|Check compile status without compiling (debug mode)

- **NOTE: Always use `--release` when building for final production**
  - Can greatly improve the size of binary
  - Also adds additional optimizations

### Check `cargo` Version

```sh
cargo --version
```

### Creating A Project

```sh
cargo new hello-cargo
```

- Creates a new project directory: `hello-cargo`

```tree
hello-cargo/
├── .git/
├── .gitignore
├── Cargo.toml
└── src/
    └── main.rs
```

Folder or File|Description
:-|:-
`.git`|Default *Version Control System (VCS)*<br>Not generated if already within a Git project<br>Can override with `--vcs` flag
`.gitignore`|Default *Version Control System (VCS)* is Git<br>Not generated if already within a Git project<br>Can override with `--vcs` flag
`Cargo.toml`|Manage project configs and dependencies/crates
`src/main.rs`|The entrance of the program

- **NOTE: Git-related files are not generated if already within an existing Git repository**
- `Cargo.toml` can have multiple sections but the following are defaulted
  - **`[package]`**
    - **Configure the project as a package**
    - Also adds compiler info
      - `name` - Name of the Package
      - `version` - Version of the Package
      - `edition` - The Rust edition to use
    - See more keys and their definitions at [Cargo Reference](https://doc.rust-lang.org/cargo/reference/manifest.html)
    - See more details about Rust Editions at [The Rust Edition Guide](https://doc.rust-lang.org/edition-guide/editions/)
  - **`[dependencies]`**
    - **List of crates dependencies for the project**
    - If none is listed, then the project will install no additional dependency crates

```toml
[package]
name = "hello-cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

- **All source codes should live in `src`**
  - Except for non-source code files, which can be in top-level project directory:
    - README files
    - License information
    - Configuration files: `Cargo.toml`, `Cargo.lock`
    - Anything else not related to code
- **Cargo helps organize projects**
  - There is a place for everything
  - Everything is in its place
- ***It is possible to convert a manually-managed project to Cargo project***
  - Move the project code into the `src` directory
  - Create an appropriate `Cargo.toml` file: Can be done by running `cargo init`

```rs
/*****************************************/
/* Print "Hello, world!" to the console. */
/*****************************************/

/// The entry-point of the program.
fn main() {
    println!();
    println!("Hello world!");
    println!();
}

// Check:               $ cargo check
// Build:               $ cargo build
// Build + Run:         $ cargo run
// Execute:             $ ./target/debug/hello-cargo
// Build Release:       $ cargo build --release
// Build + Run Release: $ cargo run --release
// Execute Release:     $ ./target/release/hello-cargo
```

### Building A Project

- Simply `cd` into the project and run `cargo build`

```sh
cd hello-cargo
cargo build
```

- It builds from `src` folder
  - **Default build is *Debug Build* `dev [unoptimized + debuginfo]`**
  - Creates an executable file in `target/debug`
- First-time build also creates `Cargo.lock`
  - Keeps track of the exact versions of dependencies/crates in the project
  - *Managed by Cargo: This file is automatically generated by Cargo*
  - ***Never need to change this file manually: It is not intended for manual editing***

#### Build And Run At Once

- We can run `cargo run` to build and run at once
- Convenience for *`cargo build` + run binaries*

```sh
cd hello-cargo
cargo run
```

- **NOTE: Cargo will only rebuild if there are diff changes detected**
- **NOTE: We can also use `cargo run --release` for the release option**

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

- **Default build is *Debug Build* (`dev [unoptimized + debuginfo]`)**
- **To build for release, we use the `--release` option**

```sh
cd hello-cargo
cargo build --release
```

- Compiles with optimizations: Binary size is much smaller
- Creates an executable file in `target/release`
- Makes Rust code run faster
- But longer build-time is needed
- **Only use this for building the final program**

### Cargo As Convention

- Cargo is invaluable for large and complex projects
- Additional tooling for project management
- We can use `cargo build` on any project made with `cargo`

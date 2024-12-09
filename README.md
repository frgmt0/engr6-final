## Script Overview
This program generates random numbers (integers or floats) and writes them to a file. While this might seem simple, it demonstrates several key programming concepts and shows how Rust handles common programming tasks differently (and often more safely) than C.

## Why Rust? Comparing with C
Coming from C, you'll notice several key differences that make Rust interesting:

1. **Memory Safety Without Garbage Collection**
   - In C, you manage memory manually with malloc/free
   - In Rust, the compiler tracks ownership and automatically frees memory
   - No more dangling pointers or memory leaks!

2. **Modern Error Handling**
   - Instead of return codes (-1, etc.) like in C, Rust uses Result types that force you to handle errors
   - Can't accidentally forget to check for errors

3. **Safe Concurrency**
   - No data races possible due to ownership system
   - Thread safety enforced at compile time
   - Much harder to create bugs than with C's threads

4. **Zero-Cost Abstractions**
   - High-level features with no runtime overhead
   - As fast as equivalent C code
   - Safer without sacrificing performance

## Why Rust? In my Opinion
Rust is a great language to learn because it's simple yet powerful. It's a good choice for learning programming concepts and applying them to real-world problems. It is also fast and relatively easy to understand syntax wise, which makes it perfect to share with others.

## Core Programming Concepts Used
Even though this is a simple program, it uses several fundamental concepts:

- **File I/O**: Just like fopen/fwrite in C, but with built-in error handling
- **Random Number Generation**: Using Rust's rand crate (similar to rand() in C)
- **User Input**: Safe string handling without buffer overflow risks
- **Error Handling**: Using Result types instead of error codes
- **Type Safety**: Strong type system prevents many common bugs

---

# Installation Guide (if you're insterested in trying it out)
All of this can be found from the [Rust website](https://www.rust-lang.org/learn/get-started)

### 1. Install Rust
First, you'll need to install Rust. It's easy:

#### Windows:
- Download rustup-init.exe from https://rustup.rs/
- Run it and follow the prompts

#### Mac/Linux:
- Open terminal and run:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Verify Installation
Open a new terminal/command prompt and type:
```bash
rustc --version
cargo --version
```
You should see version numbers for both.

### 3. Create a New Project
```bash
cargo new random_data_generator
cd random_data_generator
```

### 4. Add Dependencies
Replace the contents of `Cargo.toml` with:
```toml
[package]
name = "random_data_generator"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
```
`rand` is a crate for generating random numbers

### 5. Add the Code
Replace src/main.rs with the provided code from this repository or clone it:
```bash
git clone https://github.com/frgmt0/engr6-final.git
```

### 6. Build and Run
```bash
cargo build    # Compile the program
cargo run      # Run it
```

## Using the Program

1. When you run the program, you'll see a menu:
   ```
   1. Create new data file
   2. Exit
   ```

2. Choose option 1 to create a file. You'll need to specify:
   - Data type (i for integer, f for float)
   - Number of elements
   - Filename

3. The program will create a file containing:
   - A header line with the count
   - Random numbers in the range -1000 to 1000
   - For floats, numbers are rounded to 3 decimal places

Example output file:
```
Count: 5
342
-123
567
-890
234
```

## Common Issues and Solutions

1. **"command not found: cargo"**
   - Restart your terminal after installation
   - Make sure Rust is in your PATH

2. **Build fails**
   - Make sure Cargo.toml has the rand dependency
   - Try `cargo clean` then `cargo build`

3. **Permission denied when creating files**
   - Make sure you have write permissions in the directory

## Learning More
- Official Rust Book: https://doc.rust-lang.org/book/
- Rust By Example: https://doc.rust-lang.org/rust-by-example/
- Learn Rust With Entirely Too Many Linked Lists: https://rust-unofficial.github.io/too-many-lists/

This project is a great starting point for learning Rust, especially if you're coming from C. It shows how Rust maintains the performance of low-level languages while adding modern safety features and ergonomics.
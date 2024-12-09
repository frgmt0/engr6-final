/*
Jason Weiss 

Write a program that generates random data and writes it to a file.
The user should be able to specify the type of data (integer or float) and the number of elements to generate.
The program should then create a file with the specified name and write the data to it.


Why I did this in Rust instead of C/C++:
- It is one of the more comfortable languages for me to write in
- The syntax is fairly similar to C and python which means, to some extent, the concepts transfer over easily
- Rust is a statically typed language, which means you don't have to worry about memory management. There is no need for a garbage collector
- Rust has a built-in error handling system which makes it easier to handle errors


*/

// Similar to C you have to declare your libraries or package imports first
// In C we'd use #include, here we use these 'use' statements - they're more specific about what we import
use std::fs::File; // Like FILE* in C but with Rust's safety features
use std::io::{self, Write, BufWriter}; // How we handle I/O, like stdio.h in C
use rand::Rng; // For random numbers - external package, like linking to a lib in C
use std::io::BufRead; // For buffered reading, makes input faster


// Rust needs types for constants, unlike C where you could just #define
const MIN_VALUE: i32 = -1000; // i32 is like int in C, but explicitly 32-bit
const MAX_VALUE: i32 = 1000;

// This lets us print the enum for debugging - in C we'd have to write our own print function
#[derive(Debug)]
enum DataType {
    Integer,
    Float,
} // More powerful than C enums - you'll see how we use it with pattern matching later


// Instead of returning int like in C, we return Result
// Result is like having a built-in error code system, but safer
fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();
    
    loop {
        display_menu();
        // match is like switch in C but needs to handle all cases
        match get_choice()? {
            1 => {
                match create_file(&mut rng) {
                    Ok(_) => println!("File created successfully!"),
                    Err(e) => println!("Error creating file: {}", e),
                }
            },
            2 => break,
            _ => println!("Invalid choice!"), 
        }
    }
    
    println!("Program terminated.");
    Ok(()) // Like return 0 in C, but wrapped in Ok() to show success
}

// Simple menu display - println! is nicer than printf because it handles types automatically
fn display_menu() {
    println!("\n1. Create new data file");
    println!("2. Exit");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}


// In C we'd return -1 for errors. Here we use Result to handle success/failure
fn get_choice() -> io::Result<i32> {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input)?; // ? is a shorthand for error handling
    Ok(input.trim().parse().unwrap_or(-1)) 
}

// In C we might use chars for this. Rust uses pattern matching which is cleaner
fn get_data_type() -> io::Result<DataType> {
    print!("Enter data type (i for integer, f for float): ");
    io::stdout().flush()?; 
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    // Pattern matching is like a super-powered switch statement
    match input.trim().to_lowercase().chars().next() {
        Some('i') => Ok(DataType::Integer),
        Some('f') => Ok(DataType::Float),
        _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid data type")),
    }
}

// Gets a positive number from user - u32 is like unsigned int
fn get_element_count() -> io::Result<u32> {
    print!("Enter number of elements: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    input.trim().parse::<u32>()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid number"))
}

// String in Rust is different from char* in C
// They're UTF-8 and can't be null, so no buffer overflows
fn get_filename() -> io::Result<String> {
    print!("Enter filename: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}


// &mut is like pointers in C but Rust checks that we use them safely
// No dangling pointers or double frees!
fn create_file(rng: &mut rand::rngs::ThreadRng) -> io::Result<()> {
    let data_type = get_data_type()?;
    let count = get_element_count()?;
    let filename = get_filename()?;
    
    let file = File::create(&filename)?;
    let mut writer = BufWriter::new(file);
    
    writeln!(writer, "Count: {}", count)?;
    
    match data_type { 
        DataType::Integer => {
            for _ in 0..count {  // Nicer than C-style for loops
                let num = rng.gen_range(MIN_VALUE..=MAX_VALUE);
                writeln!(writer, "{}", num)?;
            }
        },
        DataType::Float => {
            for _ in 0..count {
                let num = rng.gen_range(MIN_VALUE as f32..=MAX_VALUE as f32);
                let num = (num * 1000.0).round() / 1000.0;
                writeln!(writer, "{:.3}", num)?;
            }
        },
    }
    
    writer.flush()?;
    Ok(())
}

// Helper for getting input - &str is like const char* in C
// but it can't be null and Rust knows its length
fn read_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}


// Main differences from C:
/*
- No manual memory management because Rust tracks who owns what
- Result type instead of error codes which makes it way harder to forget to handle errors
- Pattern matching with match is like switch but catches all cases
- Strings are always valid UTF-8 and can't be null
- Variables are immutable by default (like const in C but for every variable)
- References (&) are like pointers but Rust makes sure they're valid
*/
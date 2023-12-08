use std::io;
// create a program that prompts for your name and prints a greeting using your name.

// name
// greeting
// prompt
// print

// Inputs: name
// Processes: greeting
// Outputs: greeting

// Test
// Inputs: name = Bob
// Expected Outputs: Hello, Bob, nice to meet you!

// Inputs: name = Alice
// Expected Outputs: Hello, Alice, nice to meet you!

fn greet(name : &str) -> String {
    return format!("Hello, {name}, nice to meet you!", name = name.trim());
}

fn main() -> io::Result<()> {
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let greeting = greet(&name);
    println!("{greeting}");
    Ok(())
}
// Write a program that asks for your name, and prints "Hello, [name]
// Constraints:
//      - Keep the input, string concatenation, & output separate
//          - I guess this means deliberately not using {} in prints?
//          - Aw sick, format! exists
// Challenges:
//      - Write a new version of the program without using any variables
//          - I feel like this is deliberately made impossible in Rust
//      - Write a version of the program that displays different greetings for different people

use std::io;

fn main() {
    println!("What is your name?");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line.");

    name = name.trim().to_string();
    
    if name.eq_ignore_ascii_case("Cian") {
        name = "Smelly".to_owned()
    }

    let output = format!("Hello, {}", name);
    println!("{output}");
}
// Create a program that prompts for an input string and displays output that shows the input string
// and the number of characters the string contains
//
// Constraints:
//      - Be sure the output contains the original string
//      - Use a single output statement to construct the output
//      - Use a built-in function of the programming language to determine the length of the string
//
// Challenges:
//      - If the user enters nothing, state that the user must enter something into the program
//      - Implement this program using a GUI and update the character counter every time a key
//      is pressed. (Will do this at a later date, as I will need a hench library for this lol)

use std::io;

fn main() {
    let mut input = String::new();

    while input.len() == 0 {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        input = input.trim().to_string();

        if input.len() == 0 {
            println!("You must enter something into the program.");
        }
    }

    println!("{} is {} characters in length.", input, input.len());
}

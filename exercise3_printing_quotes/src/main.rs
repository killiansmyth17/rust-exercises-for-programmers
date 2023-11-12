// Create a program that prompts for a quote and an author
// Display the quote and author as follows:
//      [author] says, "[quote]"
//
// Constraints:
//      - Use a single output statement to produce this output
//      - If your language supports string interpolation or substitution, don't use it for this
//      exercise. Use string concatenation instead.
//
// Challenges:
//      - In Chapter 7, Data Structures, on page 63, you'll practice working with lists of data.
//      Modify this program so that instead of prompting for quotes from the user, you create a
//      structure that holds quotes and their associated attributions and then display all of the
//      quotes using the format in the example. An array of maps would be a good choice.
//          - I think he means a structure like:
//               [{author: "Some guy", quote: "I said a thing"},
//                {author: "A different guy", quote: "I'm taking a vow of silence, you know"}]

use std::collections::HashMap;
use std::{io, mem};
use std::mem::MaybeUninit;

fn main() {
    println!("First outputting normal approach.");
    normal_approach();
    println!("Now outputting challenge approach.");
    challenge_approach();
}

fn read_line(input: &mut String) {
    io::stdin()
        .read_line(input)
        .expect("Failed to read line.");
    *input = input.trim().to_string();
}

fn normal_approach() {
    let mut quote = String::new();
    println!("What is the quote?");
    read_line(&mut quote);

    let mut author = String::new();
    println!("Who said it?");
    read_line(&mut author);

    println!("{author} says, \"{quote}\"");
}

fn new_quote(author: String, quote: String) -> HashMap<String,String> {
    let mut new_quote = HashMap::new();
    new_quote.insert("Author".to_string(), author);
    new_quote.insert("Quote".to_string(), quote);

    new_quote
}

fn challenge_approach() {
    // HashMap doesn't implement copy, so initialising an array of them is a headache
    // Create uninitialised array of them with MaybeUninit
    // Use of assume_init is safe here because the type we are claiming to have initialised
    // is a bunch of MaybeUninits, which do not require initialisation.
    let quotes = {
        let mut quotes: [MaybeUninit<HashMap<String, String>>; 3] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        quotes[0].write(new_quote(
            "General Kenobi".to_string(),
            "Hello, there!".to_string()
        ));
        quotes[1].write(new_quote(
            "Jetstream Sam".to_string(),
            "Let's dance".to_string()
        ));
        quotes[2].write(new_quote(
            "Shaggy".to_string(),
            "My sword is a tool of justice, little man".to_string()
        ));

        unsafe { mem::transmute::<_, [HashMap<String, String>; 3]>(quotes) }
    };

    for quote in quotes {
        println!("{} says, \"{}\"", quote["Author"], quote["Quote"]);
    }
}
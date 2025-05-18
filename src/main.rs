mod token;
mod tokenizer;
mod parser;
mod statement;

use std::io::{self, Write};
use tokenizer::Tokenizer;
use parser::Parser;

fn main() {
    println!("Welcome to the SQL parser CLI!");
    println!("Type 'exit' to quit the program.");

    // Loop to continuously accept user input
    loop {
        // Prompt user for input
        print!("\nEnter your SQL query: ");
        io::stdout().flush().unwrap(); // Ensure prompt is printed before input

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim(); // Remove trailing newline or spaces

        // Exit condition if user types 'exit'
        if input.to_lowercase() == "exit" {
            println!("Exiting the SQL parser CLI. Goodbye!");
            break;
        }

        // Handle empty input
        if input.is_empty() {
            println!("Error: Please enter a valid SQL query.");
            continue;
        }

        // Tokenize and parse the input
        let tokenizer = Tokenizer::new(input);
        let mut parser = Parser::new(tokenizer);

        // Parse and display the SQL statement
        match parser.parse_statement() {
            Some(statement) => {
                println!("\nParsed SQL statement:");
                println!("{:#?}", statement);
            }
            None => {
                println!("\nError: Failed to parse the SQL query.");
            }
        }
    }
}

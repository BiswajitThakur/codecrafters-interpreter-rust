use std::env;
use std::fs;

use codecrafters_interpreter::TokenType;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            tokenize(&file_contents);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn tokenize(input: &str) {
    let mut e = false;
    for (no, line) in input.lines().enumerate() {
        for c in line.chars() {
            match TokenType::try_from(c) {
                Ok(v) => println!("{}", v),
                Err(_) => {
                    eprintln!("[line {}] Error: Unexpected character: {}", no + 1, c);
                    e = true;
                }
            }
        }
    }
    println!("EOF  null");
    if e {
        std::process::exit(65);
    }
}

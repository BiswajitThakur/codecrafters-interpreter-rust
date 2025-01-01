use std::env;
use std::fs;

use codecrafters_interpreter::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });
    match command.as_str() {
        "tokenize" => {
            let lx = Lexer::from(file_contents.as_str());
            for token in lx {
                println!("{}", token.as_ref());
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            return;
        }
    }
}

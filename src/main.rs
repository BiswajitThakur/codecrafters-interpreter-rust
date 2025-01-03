use std::env;
use std::fs;
use std::io;

use codecrafters_interpreter::Lexer;
use codecrafters_interpreter::Parser;
use codecrafters_interpreter::Token;
use codecrafters_interpreter::WithSpan;

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
    let mut exit_code = 0;
    match command.as_str() {
        "tokenize" => {
            let lx = Lexer::from(file_contents.as_str());
            for token in lx {
                let t = token.as_ref();
                if t.is_ok() {
                    println!("{}", t);
                } else {
                    exit_code = 65;
                    eprintln!("{}", t);
                }
            }
        }
        "parse" => {
            let lx = Lexer::from(file_contents.as_str());
            let tokens = lx.collect::<Vec<WithSpan<Token>>>();
            let mut parser = Parser::<io::Sink>::new(&tokens, None);
            if let Ok(v) = parser.parse() {
                println!("{}", v.get_value());
            } else {
                exit_code = 65;
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            return;
        }
    }
    std::process::exit(exit_code);
}

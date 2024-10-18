use std::env;
use std::fs;

use codecrafters_interpreter::tokenize;

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

            tk(&file_contents);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn tk(input: &str) {
    let mut has_err = false;
    let mut iter = input.chars().peekable();
    let mut line = 1;
    loop {
        let r = tokenize(&mut iter, &mut line);
        match r {
            Ok(Some(v)) => println!("{}", v),
            Ok(None) => break,
            Err(e) => {
                has_err = true;
                eprintln!("{}", e);
            }
        }
    }
    println!("EOF  null");
    if has_err {
        std::process::exit(65);
    }
}

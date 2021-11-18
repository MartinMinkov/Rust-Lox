pub mod scanner;
pub mod tokens;

use scanner::Scanner;
use std::env;
use std::fs;
use std::io;
use std::process;
use tokens::Token;

struct Error {
    has_error: bool,
}

impl Error {
    fn new() -> Self {
        Self { has_error: false }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let error = Error::new();
    if args.len() > 2 {
        println!("Usage: rust-lox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1], &error);
    } else {
        run_prompt(&error);
    }
}

fn run_file(path: &str, _error: &Error) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    run(contents);
}

fn run_prompt(_error: &Error) {
    loop {
        println!("> ");
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => run(line),
            Err(_) => break,
        }
    }
}

fn run(source: std::string::String) {
    let new_source = source.clone();
    let mut scanner = Scanner::new(new_source);
    scanner.scan_tokens();
    println!("Returned {} of tokens", scanner.tokens.len());

    for i in scanner.tokens {
        println!("Token: {}", i.typ);
    }
}

fn error(line: u16, message: &str) {
    report(line, "", message);
}

fn report(line: u16, error_location: &str, message: &str) {
    println!("[line '{}'] Error {} : {}", line, error_location, message);
}

mod common;
mod interpreter;
mod parsing;
mod scanner;

use common::Error;
use interpreter::Interpreter;
use parsing::Parser;
use scanner::Scanner;
use std::env;
use std::fs;
use std::io;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rust-lox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    run(contents, false);
}

fn run_prompt() {
    loop {
        println!("> ");
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => run(line, true),
            Err(_) => break,
        }
    }
}

fn run(source: std::string::String, run_in_repl: bool) {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();

    if scanner.had_error {
        process::exit(65);
    }

    let mut parser = Parser::new(scanner.tokens);
    let statements = parser.parse();
    let mut interpreter = Interpreter::new();
    for statement in statements {
        match interpreter.evaluate_statement(statement, run_in_repl) {
            Ok(()) => {}
            Err(err) => {
                Error::error(err.line, err.message);
            }
        }
    }
}

#[warn(unused_variables)]
mod parsing;
mod scanner;

use parsing::Expression;
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
    Expression::test_expr();
}

fn run_file(path: &str) {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    run(contents);
}

fn run_prompt() {
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

    if scanner.errors.len() > 0 {
        for err in scanner.errors {
            err.report();
        }
        process::exit(65);
    }
    println!("Returned {} of tokens", scanner.tokens.len());

    for token in scanner.tokens {
        println!("Token: {}", token.typ);
    }
}

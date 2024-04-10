use std::cmp::Ordering;
use std::env;
use std::fs;
use std::process::exit;

mod ast_printer;
mod error;
mod expr;
mod interpreter;
mod keyword;
mod literal;
mod parser;
mod scanner;
mod token;

use crate::ast_printer::AstPrinter;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::Token;

fn run(src: String) {
    let mut scanner: Scanner = Scanner::new(src);
    let tokens: &Vec<Token> = scanner.scan_tokens();
    let mut parser: Parser = Parser::new(tokens);

    match parser.parse() {
        Ok(expr) => {
            match Interpreter::interpret(&expr) {
                Ok(val) => match val {
                    Some(val) => println!("{}", val),
                    None => println!("None"),
                },
                Err(e) => println!("{}", e),
            }
        }
        Err(e) => println!("{}", e),
    };
}

fn run_file(path: &String) {
    let content = fs::read_to_string(path).expect("Read error!");

    run(content);
}

fn run_prompt() {
    loop {
        let src = rprompt::prompt_reply("> ").unwrap();

        run(src)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len().cmp(&2) {
        Ordering::Greater => {
            println!("Usage: rlox [script]");
            exit(64);
        }
        Ordering::Equal => run_file(&args[1]),
        Ordering::Less => run_prompt(),
    }
}

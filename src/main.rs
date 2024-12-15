use std::cmp::Ordering;
use std::env;
use std::fs;
use std::process::exit;

mod ast_printer;
mod environment;
mod error;
mod expr;
mod interpreter;
mod keyword;
mod literal;
mod parser;
mod scanner;
mod scope;
mod stmt;
mod token;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::Token;

fn run(interpreter: &mut Interpreter, src: String) {
    let mut scanner: Scanner = Scanner::new(src);
    let tokens: &Vec<Token> = scanner.scan_tokens();
    let mut parser: Parser = Parser::new(tokens);

    interpreter.interpret(parser.parse())
}

fn run_file(interpreter: &mut Interpreter, path: &String) {
    let content = fs::read_to_string(path).expect("Read error!");

    run(interpreter, content);
}

fn run_prompt(interpreter: &mut Interpreter) {
    loop {
        let src = rprompt::prompt_reply("> ").unwrap();

        run(interpreter, src)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter: Interpreter = Interpreter::new();

    match args.len().cmp(&2) {
        Ordering::Greater => {
            println!("Usage: rlox [script]");
            exit(64);
        }
        Ordering::Equal => run_file(&mut interpreter, &args[1]),
        Ordering::Less => run_prompt(&mut interpreter),
    }
}

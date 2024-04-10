use crate::token::{Token, TokenType};
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseError {
    msg: String,
    token: Token,
}

impl ParseError {
    pub fn new(msg: String, token: Token) -> ParseError {
        ParseError { msg, token }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let f_str: String = if self.token.ttype() == TokenType::Eof {
            " at end".to_string()
        } else {
            format!(" at '{}'", self.token.lexeme())
        };

        write!(
            f,
            "{}",
            report(self.token.line(), f_str, self.msg.to_string())
        )
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    msg: String,
    token: Token,
}

impl RuntimeError {
    pub fn new(msg: String, token: Token) -> RuntimeError {
        RuntimeError { msg, token }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let f_str: String = if self.token.ttype() == TokenType::Eof {
            " at end".to_string()
        } else {
            format!(" at '{}'", self.token.lexeme())
        };

        write!(
            f,
            "{}",
            report(self.token.line(), f_str, self.msg.to_string())
        )
    }
}

pub fn report(line: usize, loc: String, message: String) -> String {
    format!("[line {}] Error{}: {}", line, loc, message)
}

pub fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}

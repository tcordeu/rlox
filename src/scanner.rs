use crate::error::error;
use crate::keyword::KEYWORDS;
use crate::token::{Token, TokenType};
use std::any::Any;
use std::rc::Rc;

pub struct Scanner {
    orig_src: String,
    src: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(src: String) -> Scanner {
        Scanner {
            orig_src: src.clone(),
            src: src.chars().collect(),
            tokens: Vec::<Token>::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;

        self.src[self.current - 1]
    }

    fn add_token(&mut self, ttype: TokenType, literal: Option<Rc<dyn Any>>) {
        self.tokens.push(Token::new(
            ttype,
            self.orig_src[self.start..self.current].to_string(),
            literal,
            self.line,
        ))
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        };

        self.src[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.src.len() {
            return '\0';
        }

        self.src[self.current + 1]
    }

    fn match_char(&mut self, expected: char) -> bool {
        let actual = self.peek();
        if actual != expected {
            return false;
        };

        self.current += 1;
        true
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            };
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string".to_string());
            return;
        }

        self.advance();
        self.add_token(
            TokenType::String,
            Some(Rc::new(
                self.orig_src[self.start + 1..self.current - 1].to_string(),
            )),
        );
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let n: f64 = self.orig_src[self.start..self.current].parse().unwrap();
        self.add_token(TokenType::Number, Some(Rc::new(n)));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text: String = self.orig_src[self.start..self.current].to_string();

        let ttype = if KEYWORDS.contains_key(&text) {
            KEYWORDS[&text]
        } else {
            TokenType::Identifier
        };

        self.add_token(ttype, None)
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                let ttype = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(ttype, None)
            }
            '=' => {
                let ttype = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(ttype, None)
            }
            '<' => {
                let ttype = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(ttype, None)
            }
            '>' => {
                let ttype = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(ttype, None)
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                };
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_alphabetic() {
                    self.identifier();
                } else {
                    error(self.line, "Unexpected character".to_string());
                }
            }
        }
    }
}

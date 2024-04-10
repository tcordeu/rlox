use std::any::Any;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
    // Single char tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Minus,
    Plus,
    Slash,
    Star,

    // One-two char tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Clone, Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Rc<dyn Any>>,
    line: usize,
}

impl Token {
    pub fn new(
        ttype: TokenType,
        lexeme: String,
        literal: Option<Rc<dyn Any>>,
        line: usize,
    ) -> Token {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }

    pub fn lexeme(&self) -> &String {
        &self.lexeme
    }

    pub fn ttype(&self) -> TokenType {
        self.ttype
    }

    pub fn literal(&self) -> &Option<Rc<dyn Any>> {
        &self.literal
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

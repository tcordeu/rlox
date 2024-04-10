use crate::token::TokenType;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static KEYWORDS: Lazy<HashMap<String, TokenType>> = Lazy::new(|| {
    HashMap::from([
        (String::from("and"), TokenType::And),
        (String::from("class"), TokenType::Class),
        (String::from("else"), TokenType::Else),
        (String::from("false"), TokenType::False),
        (String::from("for"), TokenType::For),
        (String::from("fun"), TokenType::Fun),
        (String::from("if"), TokenType::If),
        (String::from("nil"), TokenType::Nil),
        (String::from("or"), TokenType::Or),
        (String::from("print"), TokenType::Print),
        (String::from("return"), TokenType::Return),
        (String::from("super"), TokenType::Super),
        (String::from("this"), TokenType::This),
        (String::from("true"), TokenType::True),
        (String::from("var"), TokenType::Var),
        (String::from("while"), TokenType::While),
    ])
});

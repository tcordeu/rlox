use crate::literal::Literal;
use crate::token::Token;
use std::rc::Rc;

pub enum Expr {
    Binary(Rc<Expr>, Token, Rc<Expr>),
    Grouping(Rc<Expr>),
    Literal(Option<Rc<dyn Literal>>),
    Unary(Token, Rc<Expr>),
}

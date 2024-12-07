use crate::expr::Expr;
use crate::token::Token;
use std::rc::Rc;

pub enum Stmt {
    Expr(Expr),
    If(Expr, Rc<Stmt>, Option<Rc<Stmt>>),
    Print(Expr),
    Var(Token, Option<Expr>),
    Block(Vec<Stmt>),
    While(Expr, Rc<Stmt>),
}

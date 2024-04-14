use crate::error::RuntimeError;
use crate::expr::Expr;
use crate::literal::*;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};
use std::rc::Rc;

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(statements: Vec<Stmt>) {
        for s in statements {
            match Self::execute(&s) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            }
        }
    }

    fn execute(s: &Stmt) -> Result<(), RuntimeError> {
        match *s {
            Stmt::Expr(ref e) => {
                let _ = Self::eval(e);
            }
            Stmt::Print(ref e) => match Self::eval(e)? {
                Some(val) => println!("{}", val),
                None => println!("None"),
            },
        }

        Ok(())
    }

    fn eval(e: &Expr) -> Result<Option<Rc<dyn Literal>>, RuntimeError> {
        match *e {
            Expr::Literal(ref l) => Ok(l.clone()),
            Expr::Grouping(ref expr) => Self::eval(expr),
            Expr::Unary(ref token, ref expr) => {
                let right: Option<Rc<dyn Literal>> = Self::eval(expr)?;

                match token.ttype() {
                    TokenType::Minus => {
                        let r_clone = Self::unwrap_optional(right, token.clone())?;
                        let val: f64 = Self::unwrap_number(r_clone, token.clone())?;

                        Ok(Some(Rc::new(NumberLiteral::new(-val))))
                    }
                    TokenType::Bang => Ok(Some(Rc::new(BoolLiteral::new(!Self::is_truthy(right))))),
                    _ => todo!(),
                }
            }
            Expr::Binary(ref lhs, ref token, ref rhs) => {
                let left: Option<Rc<dyn Literal>> = Self::eval(lhs)?;
                let right: Option<Rc<dyn Literal>> = Self::eval(rhs)?;

                let l_clone = Self::unwrap_optional(left.clone(), token.clone())?;
                let r_clone = Self::unwrap_optional(right.clone(), token.clone())?;

                match token.ttype() {
                    TokenType::Minus => {
                        let l_val: f64 = Self::unwrap_number(l_clone, token.clone())?;
                        let r_val: f64 = Self::unwrap_number(r_clone, token.clone())?;

                        Ok(Some(Rc::new(NumberLiteral::new(l_val - r_val))))
                    }
                    TokenType::Slash => {
                        let l_val: f64 = Self::unwrap_number(l_clone, token.clone())?;
                        let r_val: f64 = Self::unwrap_number(r_clone, token.clone())?;

                        Ok(Some(Rc::new(NumberLiteral::new(l_val / r_val))))
                    }
                    TokenType::Star => {
                        let l_val: f64 = Self::unwrap_number(l_clone, token.clone())?;
                        let r_val: f64 = Self::unwrap_number(r_clone, token.clone())?;

                        Ok(Some(Rc::new(NumberLiteral::new(l_val * r_val))))
                    }
                    TokenType::Greater => {
                        let l_val: f64 = Self::unwrap_number(l_clone, token.clone())?;
                        let r_val: f64 = Self::unwrap_number(r_clone, token.clone())?;

                        Ok(Some(Rc::new(BoolLiteral::new(l_val > r_val))))
                    }
                    TokenType::GreaterEqual => {
                        let l_val: f64 = Self::unwrap_number(l_clone, token.clone())?;
                        let r_val: f64 = Self::unwrap_number(r_clone, token.clone())?;

                        Ok(Some(Rc::new(BoolLiteral::new(l_val >= r_val))))
                    }
                    TokenType::Less => {
                        let l_val: f64 = Self::unwrap_number(l_clone, token.clone())?;
                        let r_val: f64 = Self::unwrap_number(r_clone, token.clone())?;

                        Ok(Some(Rc::new(BoolLiteral::new(l_val < r_val))))
                    }
                    TokenType::LessEqual => {
                        let l_val: f64 = Self::unwrap_number(l_clone, token.clone())?;
                        let r_val: f64 = Self::unwrap_number(r_clone, token.clone())?;

                        Ok(Some(Rc::new(BoolLiteral::new(l_val <= r_val))))
                    }
                    TokenType::Plus => match l_clone.ltype() {
                        LiteralType::Number => {
                            let l_val: f64 = Self::unwrap_number(l_clone, token.clone())?;
                            let r_val: f64 = Self::unwrap_number(r_clone, token.clone())?;

                            Ok(Some(Rc::new(NumberLiteral::new(l_val + r_val))))
                        }
                        LiteralType::String => {
                            let l_val: String = Self::unwrap_string(l_clone, token.clone())?;
                            let r_val: String = Self::unwrap_string(r_clone, token.clone())?;

                            Ok(Some(Rc::new(StrLiteral::new(l_val + &r_val))))
                        }
                        _ => panic!(),
                    },
                    TokenType::EqualEqual => {
                        Ok(Some(Rc::new(BoolLiteral::new(Self::is_equal(left, right)))))
                    }
                    TokenType::BangEqual => Ok(Some(Rc::new(BoolLiteral::new(!Self::is_equal(
                        left, right,
                    ))))),
                    _ => todo!(),
                }
            }
        }
    }

    fn is_truthy(l: Option<Rc<dyn Literal>>) -> bool {
        l.is_some()
    }

    fn is_equal(l: Option<Rc<dyn Literal>>, r: Option<Rc<dyn Literal>>) -> bool {
        if r.is_none() {
            return l.is_none();
        }

        let l = l.unwrap();
        let r = r.unwrap();

        if l.ltype() != r.ltype() {
            return false;
        }

        match l.ltype() {
            LiteralType::Number => {
                let l_val: &f64 = l.value().downcast_ref::<f64>().unwrap();
                let r_val: &f64 = r.value().downcast_ref::<f64>().unwrap();

                l_val == r_val
            }
            LiteralType::String => {
                let l_val: &String = l.value().downcast_ref::<String>().unwrap();
                let r_val: &String = r.value().downcast_ref::<String>().unwrap();

                l_val == r_val
            }
            _ => false,
        }
    }

    fn unwrap_optional(
        op: Option<Rc<dyn Literal>>,
        token: Token,
    ) -> Result<Rc<dyn Literal>, RuntimeError> {
        Ok(op
            .ok_or(RuntimeError::new(
                "Expect operand, None provided".to_string(),
                token.clone(),
            ))?
            .clone())
    }

    fn unwrap_number(op: Rc<dyn Literal>, token: Token) -> Result<f64, RuntimeError> {
        Ok(*op.value().downcast_ref::<f64>().ok_or(RuntimeError::new(
            "Expect number".to_string(),
            token.clone(),
        ))?)
    }

    fn unwrap_string(op: Rc<dyn Literal>, token: Token) -> Result<String, RuntimeError> {
        Ok(op
            .value()
            .downcast_ref::<String>()
            .ok_or(RuntimeError::new(
                "Expect String".to_string(),
                token.clone(),
            ))?
            .to_string())
    }
}

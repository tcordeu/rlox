use crate::environment::Environment;
use crate::error::RuntimeError;
use crate::expr::Expr;
use crate::literal::*;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};
use std::rc::Rc;

pub struct Interpreter {
    current_env: Option<Box<Environment>>,
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            current_env: None,
            env: Environment::new(None),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for s in statements {
            match self.execute(&s) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            }
        }
    }

    fn execute(&mut self, s: &Stmt) -> Result<(), RuntimeError> {
        match *s {
            Stmt::Expr(ref e) => {
                let _ = self.eval(e);
            }
            Stmt::If(ref cond, ref then_s, ref else_s) => {
                if Self::is_truthy(self.eval(cond)?) {
                    self.execute(then_s)?;
                } else {
                    self.execute(else_s.as_ref().unwrap())?;
                }
            }
            Stmt::Print(ref e) => match self.eval(e)? {
                Some(val) => println!("{}", val),
                None => println!("nil"),
            },
            Stmt::Var(ref token, ref init) => {
                let val: Option<Rc<dyn Literal>> = match init {
                    Some(n) => self.eval(n)?,
                    None => None,
                };

                self.get_env().define(token.lexeme().to_string(), val)
            }
            Stmt::Block(ref statements) => {
                let prev_env = self.current_env.clone();

                self.current_env = Some(Box::new(Environment::new(Some(Box::new(
                    self.get_env().clone(),
                )))));
                for s in statements {
                    self.execute(s)?;
                }

                self.current_env = prev_env;
            }
        }

        Ok(())
    }

    fn eval(&mut self, e: &Expr) -> Result<Option<Rc<dyn Literal>>, RuntimeError> {
        match *e {
            Expr::Literal(ref l) => Ok(l.clone()),
            Expr::Grouping(ref expr) => self.eval(expr),
            Expr::Var(ref token) => self.get_env().get(token.clone()).cloned(),
            Expr::Assign(ref token, ref expr) => {
                let val: Option<Rc<dyn Literal>> = self.eval(expr)?;

                self.get_env().assign(token.clone(), val.clone())?;
                Ok(val)
            }
            Expr::Unary(ref token, ref expr) => {
                let right: Option<Rc<dyn Literal>> = self.eval(expr)?;

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
                let left: Option<Rc<dyn Literal>> = self.eval(lhs)?;
                let right: Option<Rc<dyn Literal>> = self.eval(rhs)?;

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
        if !l.is_some() {
            return false;
        }

        if l.unwrap().ltype() == LiteralType::False {
            false
        } else {
            true
        }
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

    fn get_env(&mut self) -> &mut Environment {
        if self.current_env.is_some() {
            self.current_env.as_mut().unwrap()
        } else {
            &mut self.env
        }
    }
}

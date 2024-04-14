use crate::error::ParseError;
use crate::expr::Expr;
use crate::literal::*;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};
use std::rc::Rc;

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Parser {
        Parser {
            current: 0,
            tokens: tokens.to_vec(),
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_at_end() {
            if let Some(s) = self.declaration() {
                statements.push(s)
            }
        }

        statements
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype() == TokenType::Eof
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().ttype() == ttype
    }

    fn match_token(&mut self, ttypes: &[TokenType]) -> bool {
        for ttype in ttypes {
            if self.check(*ttype) {
                self.advance();

                return true;
            }
        }

        false
    }

    fn consume(&mut self, ttype: TokenType, msg: &str) -> Result<Token, ParseError> {
        if self.check(ttype) {
            return Ok(self.advance());
        }

        Err(ParseError::new(msg.to_string(), self.peek()))
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.match_token(&[TokenType::Var]) {
            match self.var_declaration() {
                Ok(s) => Some(s),
                Err(_) => {
                    self.sync();
                    None
                }
            }
        } else {
            match self.statement() {
                Ok(s) => Some(s),
                Err(_) => {
                    self.sync();
                    None
                }
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name: Token = self.consume(TokenType::Identifier, "Expect variable name")?;
        let init: Option<Expr> = if self.match_token(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        let _ = self.consume(TokenType::Semicolon, "Expect ';' after var declaration");
        Ok(Stmt::Var(name, init))
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::Print]) {
            Ok(self.print_statement()?)
        } else if self.match_token(&[TokenType::LeftBrace]) {
            Ok(Stmt::Block(self.block()?))
        } else {
            Ok(self.expression_statement()?)
        }
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements: Vec<Stmt> = Vec::<Stmt>::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Some(s) = self.declaration() {
                statements.push(s)
            }
        }

        let _ = self.consume(TokenType::RightBrace, "Expect '}' after block")?;
        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr: Expr = self.expression()?;
        let _ = self.consume(TokenType::Semicolon, "Expect ';' after value")?;

        Ok(Stmt::Print(expr))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr: Expr = self.expression()?;
        let _ = self.consume(TokenType::Semicolon, "Expect ';' after expression")?;

        Ok(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr: Expr = self.equality()?;

        if self.match_token(&[TokenType::Equal]) {
            let equals: Token = self.previous();
            let val: Expr = self.assignment()?;

            match expr {
                Expr::Var(ref token) => return Ok(Expr::Assign(token.clone(), Rc::new(val))),
                _ => {
                    return Err(ParseError::new(
                        "Invalid assignment target".to_string(),
                        equals,
                    ))
                }
            }
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous();
            let r_expr: Expr = self.comparison()?;
            expr = Expr::Binary(Rc::new(expr), operator, Rc::new(r_expr));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.term()?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator: Token = self.previous();
            let r_expr = self.term()?;
            expr = Expr::Binary(Rc::new(expr), operator, Rc::new(r_expr));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.previous();
            let r_expr = self.factor()?;
            expr = Expr::Binary(Rc::new(expr), operator, Rc::new(r_expr));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.unary()?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous();
            let r_expr = self.unary()?;
            expr = Expr::Binary(Rc::new(expr), operator, Rc::new(r_expr));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.previous();
            let r_expr = self.unary()?;

            return Ok(Expr::Unary(operator, Rc::new(r_expr)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Literal(Some(Rc::new(BoolLiteral::new(false)))));
        }
        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Literal(Some(Rc::new(BoolLiteral::new(true)))));
        }
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Literal(None));
        }
        if self.match_token(&[TokenType::Number]) {
            let val: f64 = *self
                .previous()
                .literal()
                .clone()
                .unwrap()
                .downcast::<f64>()
                .ok()
                .ok_or(ParseError::new("Expect number".to_string(), self.peek()))?;

            return Ok(Expr::Literal(Some(Rc::new(NumberLiteral::new(val)))));
        }
        if self.match_token(&[TokenType::Number, TokenType::String]) {
            let val: String = self
                .previous()
                .literal()
                .clone()
                .unwrap()
                .downcast::<String>()
                .ok()
                .ok_or(ParseError::new("Expect string".to_string(), self.peek()))?
                .to_string();

            return Ok(Expr::Literal(Some(Rc::new(StrLiteral::new(val)))));
        }
        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expr::Var(self.previous()));
        }
        if self.match_token(&[TokenType::LeftParen]) {
            let expr: Expr = self.expression()?;

            let _ = self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(Rc::new(expr)));
        }

        Err(ParseError::new(
            "Expect expression".to_string(),
            self.peek(),
        ))
    }

    fn sync(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().ttype() == TokenType::Semicolon {
                return;
            }

            match self.peek().ttype() {
                TokenType::Class => return,
                TokenType::Fun => return,
                TokenType::Var => return,
                TokenType::For => return,
                TokenType::If => return,
                TokenType::While => return,
                TokenType::Print => return,
                TokenType::Return => return,
                _ => self.advance(),
            };
        }
    }
}

use crate::error::ParseError;
use crate::expr::Expr;
use crate::literal::*;
use crate::stmt::*;
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

    pub fn parse(&mut self) -> Result<Vec<Rc<dyn Stmt>>, ParseError> {
        let statements: Vec<Rc<dyn Stmt>> = Vec::new();

        while !self.is_at_end() {
            statements.push(self.statement()?)
        }

        Ok(statements)
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

    fn statement(&self) -> Result<Rc<dyn Stmt>, ParseError> {
        if self.match_token(&[TokenType::Print]) {
            Ok(Rc::new(self.print_statement()?))
        } else {
            Ok(Rc::new(self.expression_statement()?))
        }
    }

    fn print_statement(&self) -> Result<PrintStmt, ParseError> {
        let expr: Expr = self.expression()?;
        let _ = self.consume(TokenType::Semicolon, "Expect ';' after value")?;

        Ok(PrintStmt::new(expr))
    }

    fn expression_statement(&self) -> Result<ExprStmt, ParseError> {
        let expr: Expr = self.expression()?;
        let _ = self.consume(TokenType::Semicolon, "Expect ';' after expression")?;

        Ok(ExprStmt::new(expr))
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
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

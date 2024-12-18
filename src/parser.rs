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
                Err(e) => {
                    self.show_error(e);
                    self.sync();
                    None
                }
            }
        } else {
            match self.statement() {
                Ok(s) => Some(s),
                Err(e) => {
                    self.show_error(e);
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
        if self.match_token(&[TokenType::For]) {
            self.for_statement()
        } else if self.match_token(&[TokenType::If]) {
            self.if_statement()
        } else if self.match_token(&[TokenType::Print]) {
            self.print_statement()
        } else if self.match_token(&[TokenType::While]) {
            self.while_statement()
        } else if self.match_token(&[TokenType::LeftBrace]) {
            Ok(Stmt::Block(self.block()?))
        } else {
            self.expression_statement()
        }
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        let _ = self.consume(TokenType::LeftParen, "Expect '(' after 'if'");
        let condition: Expr = self.expression()?;
        let _ = self.consume(TokenType::RightParen, "Expect ')' after if condition");

        let then_stmt: Stmt = self.statement()?;
        let else_stmt: Option<Rc<Stmt>> = if self.match_token(&[TokenType::Else]) {
            Some(Rc::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(condition, Rc::new(then_stmt), else_stmt))
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

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'")?;
        let condition: Expr = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition")?;
        let body: Stmt = self.statement()?;

        Ok(Stmt::While(condition, Rc::new(body)))
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        let _ = self.consume(TokenType::LeftParen, "Expect '(' after 'for'")?;

        let initializer: Option<Stmt>;
        if self.match_token(&[TokenType::Semicolon]) {
            initializer = None;
        } else if self.match_token(&[TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition: Option<Expr> = None;
        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition")?;

        let mut increment: Option<Expr> = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(TokenType::RightParen, "Expect ')' after for clauses")?;

        let mut body: Stmt = self.statement()?;

        if increment.is_some() {
            let mut stmts: Vec<Stmt> = Vec::new();
            stmts.push(body);
            stmts.push(Stmt::Expr(increment.unwrap()));
            body = Stmt::Block(stmts);
        }
        if condition.is_none() {
            condition = Some(Expr::Literal(Some(Rc::new(BoolLiteral::new(true)))));
        }
        body = Stmt::While(condition.unwrap(), Rc::new(body));

        if initializer.is_some() {
            let mut stmts: Vec<Stmt> = Vec::new();
            stmts.push(initializer.unwrap());
            stmts.push(body);
            body = Stmt::Block(stmts);
        }

        Ok(body)
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
        let expr: Expr = self.or()?;

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

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.and()?;

        while self.match_token(&[TokenType::Or]) {
            let operator: Token = self.previous();
            let right: Expr = self.and()?;

            expr = Expr::Logical(Rc::new(expr), operator, Rc::new(right));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.equality()?;

        while self.match_token(&[TokenType::And]) {
            let operator: Token = self.previous();
            let right: Expr = self.equality()?;

            expr = Expr::Logical(Rc::new(expr), operator, Rc::new(right));
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

    fn show_error(&self, e: ParseError) {
        println!("{}", e);
    }
}

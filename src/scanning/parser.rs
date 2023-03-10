use crate::*;
use scanning::token::Token;
use scanning::expr::Expr;

pub type ParseResult = Result<Expr, Error>;

pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, idx: 0 }
    }
    pub fn advance(&mut self) { self.idx += 1; }
    pub fn advance_if(&mut self, cond: bool) {
        if cond { self.advance(); }
    }
    pub fn token(&self) -> Option<&Token> {
        self.tokens.get(self.idx)
    }
    pub fn expect_token(&mut self, expected_token: Token) -> Result<(), String> {
        if let Some(token) = self.token() {
            if token != &expected_token { return error!("expected token '{expected_token}', got '{token}'") }
            Ok(())
        } else {
            error!("unexpected end, expected token '{expected_token}'")
        }
    }
    pub fn expect_token_advance(&mut self, expected_token: Token) -> Result<(), String> {
        self.expect_token(expected_token)?;
        self.advance();
        Ok(())
    }
    pub fn parse(&mut self) -> ParseResult {
        let expr = self.expr()?;
        if let Some(token) = self.token() {
            return error!("cannot handel '{token}' at the end of input")
        }
        Ok(expr)
    }
    pub fn expr(&mut self) -> ParseResult {
        self.arith()
    }
    pub fn arith(&mut self) -> ParseResult {
        let mut left = self.term()?;
        while let Some(token) = self.token() {
            if ![Token::Add, Token::Sub, Token::AddSub].contains(&token) { break }
            let op = token.clone();
            self.advance();
            let right = Box::new(self.term()?);
            left = Expr::BinaryOperation { left: Box::from(left), right, op }
        }
        Ok(left)
    }
    pub fn term(&mut self) -> ParseResult {
        let mut left = self.pow()?;
        while let Some(token) = self.token() {
            if ![Token::Mult, Token::Div].contains(&token) { break }
            let op = token.clone();
            self.advance();
            let right = Box::new(self.pow()?);
            left = Expr::BinaryOperation { left: Box::from(left), right, op }
        }
        Ok(left)
    }
    pub fn pow(&mut self) -> ParseResult {
        let mut left = self.factor()?;
        while let Some(token) = self.token() {
            if token != &Token::Power { break }
            let op = token.clone();
            self.advance();
            let right = Box::new(self.factor()?);
            left = Expr::BinaryOperation { left: Box::from(left), right, op }
        }
        Ok(left)
    }
    pub fn factor(&mut self) -> ParseResult {
        if let Some(token) = self.token() {
            if [Token::Add, Token::Sub].contains(&token) {
                let op = token.clone();
                self.advance();
                return Ok(Expr::UnaryOperation { expr: Box::from(self.fraction()?), op })
            }
        }
        self.fraction()
    }
    pub fn fraction(&mut self) -> ParseResult {
        let expr = self.percent()?;
        if self.token() == Some(&Token::Fraction) {
            self.advance();
            return Ok(Expr::UnaryOperationRight { expr: Box::new(expr), op: Token::Fraction })
        }
        Ok(expr)
    }
    pub fn percent(&mut self) -> ParseResult {
        let expr = self.atom()?;
        if self.token() == Some(&Token::Percent) {
            self.advance();
            return Ok(Expr::UnaryOperationRight { expr: Box::new(expr), op: Token::Percent })
        }
        Ok(expr)
    }
    pub fn atom(&mut self) -> ParseResult {
        let res = match self.token() {
            Some(Token::EvalIn) => {
                self.advance();
                let expr = self.expr()?;
                if self.token() != Some(&Token::EvalOut) || self.token() == Some(&Token::Seperator) {
                    self.advance_if(self.token() == Some(&Token::Seperator));
                    let mut exprs = vec![expr];
                    while let Some(token) = self.token() {
                        if token == &Token::EvalOut { break }
                        exprs.push(self.expr()?);
                        self.advance_if(self.token() == Some(&Token::Seperator));
                    }
                    self.expect_token_advance(Token::EvalOut)?;
                    return Ok(Expr::Vector(exprs))
                }
                Ok(expr)
            }
            Some(Token::Pipe) => {
                self.advance();
                let expr = Box::new(self.expr()?);
                self.expect_token_advance(Token::Pipe)?;
                Ok(Expr::Absolute(expr))
            }
            Some(Token::SetIn) => {
                self.advance();
                let mut exprs = vec![];
                while let Some(token) = self.token() {
                    if token == &Token::SetOut { break }
                    exprs.push(self.expr()?);
                    self.advance_if(self.token() == Some(&Token::Seperator));
                }
                self.expect_token_advance(Token::SetOut)?;
                Ok(Expr::Set(exprs))
            }
            Some(Token::ID(id)) => Ok(Expr::ID(id.clone())),
            Some(Token::Int(v)) => Ok(Expr::Int(*v)),
            Some(Token::Float(v)) => Ok(Expr::Float(*v)),
            Some(Token::Continue) => Ok(Expr::Continue),
            Some(token) => error!("unexpected token '{token}'"),
            None => error!("unexpected end of input")
        };
        if res.is_ok() { self.advance(); }
        res
    }
}

pub fn parse(tokens: Vec<Token>) -> ParseResult {
    Parser::new(tokens).parse()
}
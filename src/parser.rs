use std::fmt::{Error as FmtError, Formatter};
use crate::lexer::{Token, TokenKind};

#[derive(Debug)]
pub enum Error {
  ParseError,
  UnexpectedToken,
  UnexpectedBinaryOperator,
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::ParseError => write!(f, "Parse error"),
      Error::UnexpectedToken => write!(f, "Unexpected token"),
      Error::UnexpectedBinaryOperator => write!(f, "Unexpected binary operator"),
    }
  }
}

pub enum BinaryOperator {
  Add,
  Subtract,
  Multiply,
  Divide,
  Power,
  Modulo,
}

impl std::fmt::Debug for BinaryOperator {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      BinaryOperator::Add => write!(f, "+"),
      BinaryOperator::Subtract => write!(f, "-"),
      BinaryOperator::Multiply => write!(f, "*"),
      BinaryOperator::Divide => write!(f, "/"),
      BinaryOperator::Power => write!(f, "^"),
      BinaryOperator::Modulo => write!(f, "%"),
    }
  }
}

#[derive(Debug)]
pub enum Expr {
  NumberExpr(f64),
  BinaryOperatorExpr(Box<Expr>, BinaryOperator, Box<Expr>),
}

pub struct Parser<'a> {
  cursor: usize,
  tokens: Vec<Token<'a>>
}

impl<'a> Parser<'a> {
  pub fn get_binary_operator(kind: TokenKind) -> Result<BinaryOperator, Error> {
    match kind {
      TokenKind::Plus => Ok(BinaryOperator::Add),
      TokenKind::Minus => Ok(BinaryOperator::Subtract),
      TokenKind::Star => Ok(BinaryOperator::Multiply),
      TokenKind::Slash => Ok(BinaryOperator::Divide),
      TokenKind::StarStar => Ok(BinaryOperator::Power),
      TokenKind::Percent => Ok(BinaryOperator::Modulo),
      _ => Err(Error::UnexpectedBinaryOperator),
    }
  }

  pub fn new(tokens: Vec<Token<'a>>) -> Self {
    Self {
      cursor: 0,
      tokens,
    }
  }

  fn eat(&mut self, kind: TokenKind) -> Result<(), Error> {
    let curr = self.curr();
    if curr.kind == kind {
      self.cursor += 1;
      Ok(())
    } else {
      Err(Error::UnexpectedToken)
    }
  }

  fn curr(&self) -> Token<'a> {
    if self.cursor >= self.tokens.len() {
      return Token::new(TokenKind::EOF, "");
    }

    self.tokens[self.cursor]
  }

  fn factor(&mut self) -> Result<Expr, Error> {
    let curr = self.curr();

    match curr.kind {
      TokenKind::Number => {
        return match curr.slice.parse() {
          Ok(n) => {
            self.eat(TokenKind::Number)?;
            Ok(Expr::NumberExpr(n))
          },
          Err(_) => Err(Error::ParseError),
        };
      },
      TokenKind::OpenParen => {
        self.eat(TokenKind::OpenParen)?;
        let expr = self.expr()?;
        self.eat(TokenKind::CloseParen)?;
        Ok(expr)
      },
      _ => {
        Err(Error::UnexpectedToken)
      }
    }
  }

  fn term(&mut self) -> Result<Expr, Error> {
    let mut node = self.factor()?;

    while
      self.curr().kind == TokenKind::Star ||
      self.curr().kind == TokenKind::Slash ||
      self.curr().kind == TokenKind::StarStar ||
      self.curr().kind == TokenKind::Percent
    {
      let token = self.curr();
      self.eat(token.kind)?;

      node = Expr::BinaryOperatorExpr(
        Box::new(node),
        Parser::get_binary_operator(token.kind)?,
        Box::new(self.factor()?)
      );
    }

    Ok(node)
  }

  fn expr(&mut self) -> Result<Expr, Error> {
    let mut node = self.term()?;

    while self.curr().kind == TokenKind::Plus || self.curr().kind == TokenKind::Minus {
      let token = self.curr();
      self.eat(token.kind)?;

      node = Expr::BinaryOperatorExpr(
        Box::new(node),
        Parser::get_binary_operator(token.kind)?,
        Box::new(self.term()?)
      );
    }

    Ok(node)
  }

  pub fn parse(&mut self) -> Result<Expr, Error> {
    Ok(self.expr()?)
  }
}
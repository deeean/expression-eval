use regex::Regex;
use crate::lexer::{Lexer, Rule, TokenKind};
use crate::parser::{BinaryOperator, Error, Expr, Parser};

pub struct Evaluator {

}

impl Evaluator {
  fn visit(expr: &Expr) -> f64 {
    match expr {
      Expr::NumberExpr(n) => {
        return n.clone()
      },
      Expr::BinaryOperatorExpr(left, operator, right) => {
        let left = Evaluator::visit(left);
        let right = Evaluator::visit(right);
        match operator {
          BinaryOperator::Add => left + right,
          BinaryOperator::Subtract => left - right,
          BinaryOperator::Multiply => left * right,
          BinaryOperator::Divide => left / right,
          BinaryOperator::Power => left.powf(right),
          BinaryOperator::Modulo => left % right,
        }
      }
    }
  }

  pub fn evaluate(source: &str) -> Result<f64, Error> {
    let lexer = Lexer::new(vec![
      Rule::with_regex(TokenKind::Number, Regex::new(r"-?[0-9]+\.?[0-9]*").unwrap()),
      Rule::with_keyword(TokenKind::Plus, "+"),
      Rule::with_keyword(TokenKind::Minus, "-"),
      Rule::with_keyword(TokenKind::StarStar, "**"),
      Rule::with_keyword(TokenKind::Star, "*"),
      Rule::with_keyword(TokenKind::Slash, "/"),
      Rule::with_keyword(TokenKind::Percent, "%"),
      Rule::with_keyword(TokenKind::OpenParen, "("),
      Rule::with_keyword(TokenKind::CloseParen, ")"),
    ]);

    let tokens = lexer.parse(source);
    let expr = Parser::new(tokens).parse()?;

    Ok(Evaluator::visit(&expr))
  }
}
use regex::Regex;
use expression_parser::evaluator::Evaluator;

fn main() {
  match Evaluator::evaluate("cos(1 + 2)") {
    Ok(result) => println!("{}", result),
    Err(error) => println!("{}", error),
  }
}

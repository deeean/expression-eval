use regex::Regex;

#[derive(Debug, Copy, Clone)]
pub struct Token<'a> {
  pub kind: TokenKind,
  pub slice: &'a str,
}

impl<'a> Token<'a> {
  pub fn new(kind: TokenKind, slice: &'a str) -> Self {
    Self { kind, slice }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
  Number,

  Plus,
  Minus,
  Star,
  Slash,
  StarStar,
  Percent,

  OpenParen,
  CloseParen,

  EOF
}

pub struct Rule<'a> {
  pub kind: TokenKind,
  pub regex: Option<Regex>,
  pub keyword: Option<&'a str>,
}

impl<'a> Rule<'a> {
  pub fn with_regex(kind: TokenKind, regex: Regex) -> Self {
    Self {
      kind,
      regex: Some(regex),
      keyword: None,
    }
  }

  pub fn with_keyword(kind: TokenKind, keyword: &'a str) -> Self {
    Self {
      kind,
      regex: None,
      keyword: Some(keyword),
    }
  }
}

pub struct Lexer<'a> {
  rules: Vec<Rule<'a>>
}

impl<'a> Lexer<'a> {
  pub fn new(rules: Vec<Rule<'a>>) -> Self {
    Self {
      rules,
    }
  }

  pub fn token<'b>(&self, slice: &'b str) -> Option<Token<'b>> {
    for rule in &self.rules {
      if let Some(regex) = &rule.regex {
        let matches = match regex.captures(slice) {
          Some(captures) => captures.get(0),
          None => continue,
        };

        let res = match matches {
          Some(m) => m,
          None => continue,
        };

        if res.start() != 0 {
          continue;
        }

        return Some(Token::new(rule.kind, &slice[res.start()..res.end()]));
      } else if let Some(keyword) = &rule.keyword {
        if slice.len() < keyword.len() {
          continue;
        }

        let slice2 = &slice[..keyword.len()];
        if *keyword != slice2 {
          continue;
        }

        return Some(Token::new(rule.kind, slice2));
      }
    }

    return None
  }

  pub fn parse(&self, source: &'a str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut cursor = 0_usize;
    let len = source.len();

    while cursor < len {
      let slice = &source[cursor..];
      let token = self.token(slice);
      match token {
        Some(token) => {
          cursor += token.slice.len();
          tokens.push(token);
        },
        None => {
          cursor += 1;
        }
      }
    }

    tokens
  }
}
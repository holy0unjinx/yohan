#[derive(Debug, PartialEq)]
pub enum Token {
  Number(i64),
  Plus,
  Minus,
  Divide,
  Multiply,
  LParen,
  RParen,
  EOF,
}

pub struct Lexer<'a> {
  input: &'a str,
  position: usize,
  current_char: Option<char>,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Lexer<'a> {
    let mut lexer = Lexer {
      input,
      position: 0,
      current_char: None,
    };
    lexer.current_char = Some(lexer.input.chars().nth(lexer.position).unwrap());
    lexer
  }

  fn advance(&mut self) {
    self.position += 1;
    self.current_char = if self.position < self.input.len() {
      Some(self.input.chars().nth(self.position).unwrap())
    } else {
      None
    };
  }

  fn skip_whitespace(&mut self) {
    while let Some(c) = self.current_char {
      if c.is_whitespace() {
        self.advance();
      } else {
        break;
      }
    }
  }

  fn number(&mut self) -> Token {
    let start_pos = self.position;
    while let Some(c) = self.current_char {
      if c.is_digit(10) {
        self.advance();
      } else {
        break;
      }
    }
    let number_str = &self.input[start_pos..self.position];
    Token::Number(number_str.parse().unwrap())
  }

  pub fn get_next_token(&mut self) -> Token {
    while let Some(c) = self.current_char {
      match c {
        ' ' | '\t' | '\n' | '\r' => {
          self.skip_whitespace();
          continue;
        }
        '0'..='9' => return self.number(),
        '+' => {
          self.advance();
          return Token::Plus;
        }
        '-' => {
          self.advance();
          return Token::Minus;
        }
        '*' => {
          self.advance();
          return Token::Multiply;
        }
        '/' => {
          self.advance();
          return Token::Divide;
        }
        '(' => {
          self.advance();
          return Token::LParen;
        }
        ')' => {
          self.advance();
          return Token::RParen;
        }
        _ => {
          panic!("Invalid character: {}", c);
        }
      }
    }
    Token::EOF
  }
}
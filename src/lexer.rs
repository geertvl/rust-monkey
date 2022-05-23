use crate::token::{Token, TokenType};


pub struct Lexer {
  pub input: String,
  pub position: i32,
  pub read_position: i32,
  pub ch: char,
}

impl Lexer {
  pub fn new(input: String) -> Lexer {
    let mut l = Lexer {
      input: input,
      position: 0,
      read_position: 0,
      ch: '\0'
    };
    l.read_char();

    l
  }

  pub fn read_char(&mut self) {
    if self.read_position >= self.input.len().try_into().unwrap() {
      self.ch = '\0'
    } else {
      // TODO: should this be in a separate function that checks the Option?
      self.ch = self.input.chars().nth(self.read_position.try_into().unwrap()).unwrap();
    }
    self.position = self.read_position;
    self.read_position += 1;
  }

  pub fn next_token(&self) -> Token {
    match self.ch {
      ' ' => Token { token_type: TokenType::Assign, literal: self.ch },
      ';' => Token { token_type: TokenType::SemiColon, literal: self.ch },
      // TODO : literal should be an empty char
      _   => Token { token_type: TokenType::Eof, literal: '\0' }
    }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use crate::token::TokenType;
  use crate::lexer::Lexer;

  #[test]
  fn next_token() {
      let input = "=+(){},;";

      let mut test_input = HashMap::from([
        (TokenType::Assign, '=')
      ]);

      let l = Lexer::new(input.to_string());

      for (tt, lit) in test_input {
        let tok = l.next_token();
        assert_eq!(tok.token_type, tt);
        assert_eq!(tok.literal, lit);
      }
  }
} 
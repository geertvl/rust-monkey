use crate::token::TokenType;

pub struct Lexer {
  pub input: String,
  pub position: i32,
  pub read_position: i32,
  pub ch: char,
}

impl Lexer {
  pub fn new(input: String) -> Lexer {
    Lexer {
      input: input,
      position: 0,
      read_position: 0,
      ch: '\0'
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

      let mut t = HashMap::from([
        (TokenType::Assign, "=")
      ]);

      let l = Lexer::new(input.to_string());

      for 
  }
} 
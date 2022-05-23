use std::cmp::Eq;

#[derive(Hash)]
pub enum TokenType {
  Illegal,
  Eof,
  Ident,
  Int,
  Assign,
  Plus,
  Lparen,
  Rparen,
  Lbrace,
  Rbrace,
  Function,
  Let
} 

impl PartialEq for TokenType {
  fn eq(&self, other: &Self) -> bool {
    self == other
  }
}

impl Eq for TokenType {}
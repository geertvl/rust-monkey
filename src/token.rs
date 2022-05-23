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
  Let,
  Comma,
  SemiColon
} 

impl PartialEq for TokenType {
  fn eq(&self, other: &Self) -> bool {
    self == other
  }
}

impl Eq for TokenType {}

pub struct Token {
  pub token_type: TokenType,
  pub literal: char,
}
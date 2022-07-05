use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
  SemiColon,
  Minus,
  Bang,
  Asterisk,
  Slash,
  Lt,
  Gt,
  True,
  False,
  If,
  Else,
  Return,
  Eq,
  NotEq,
} 

impl PartialEq<TokenType> for Token {
  fn eq(&self, other: &TokenType) -> bool {
    self.token_type == *other
  }
}

impl PartialEq<Token> for TokenType {
  fn eq(&self, other: &Token) -> bool {
    *self == other.token_type
  }
}

pub struct Token {
  pub token_type: TokenType,
  pub literal: String,
}

impl fmt::Debug for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Token")
      .field("token_type", &self.token_type)
      .field("literal", &self.token_type)
      .finish()
  }
}

impl Clone for Token {
  fn clone(&self) -> Self {
    Token {
      token_type: self.token_type.clone(),
      literal: self.literal.clone(),
    }
   }
}

pub struct Keywords {}

impl Keywords {
  pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
      "fn" => TokenType::Function,
      "let" => TokenType::Let,
      "true" => TokenType::True,
      "false" => TokenType::False,
      "if" => TokenType::If,
      "else" => TokenType::Else,
      "return" => TokenType::Return,
      _ => TokenType::Ident,
    }
  }
}
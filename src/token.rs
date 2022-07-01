

#[derive(Debug, PartialEq, Eq, Hash)]
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
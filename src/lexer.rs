use crate::token::{Token, TokenType, Keywords};
use crate::support::{convert_u32, convert_usize, is_letter, is_digit};
use substring::Substring;

pub struct Lexer {
  pub input: String,
  pub position: u32,
  pub read_position: u32,
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

  fn read_char(&mut self) {
    let length = convert_u32(self.input.len());
    if self.read_position >= length {
      self.ch = '\0';
    } else {
      let pos = convert_usize(self.read_position);
      self.ch = self.input.chars().nth(pos).unwrap();
    }

    self.position = self.read_position;
    self.read_position += 1;
  }

  fn read_identifier(&mut self) -> String {
    let position = self.position;
    while is_letter(self.ch) {
      self.read_char();
    }

    let start_pos = convert_usize(position);
    let end_pos = convert_usize(self.position);
    self.input.substring(start_pos, end_pos).to_string()
  }

  fn read_number(&mut self) -> String {
    let position = self.position;
    while is_digit(self.ch) {
      self.read_char();
    }

    let start_pos = convert_usize(position);
    let end_pos = convert_usize(self.position);
    self.input.substring(start_pos, end_pos).to_string()
  }

  fn skip_whitespace(&mut self) {
    while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
      self.read_char();
    }
  }

  pub fn next_token(&mut self) -> Token {
    self.skip_whitespace();

    let tok = match self.ch {
        '=' => Token {
          token_type: TokenType::Assign,
          literal: self.ch.to_string(),
        },
        ';' => Token {
          token_type: TokenType::SemiColon,
          literal: self.ch.to_string(),
        },
        '(' => Token {
          token_type: TokenType::Lparen,
          literal: self.ch.to_string(),
        },
        ')' => Token {
          token_type: TokenType::Rparen,
          literal: self.ch.to_string(),
        },
        ',' => Token {
          token_type: TokenType::Comma,
          literal: self.ch.to_string(),
        },
        '+' => Token {
          token_type: TokenType::Plus,
          literal: self.ch.to_string(),
        },
        '-' => Token {
          token_type: TokenType::Minus,
          literal: self.ch.to_string(),
        },
        '!' => Token {
          token_type: TokenType::Bang,
          literal: self.ch.to_string(),
        },
        '/' => Token {
          token_type: TokenType::Slash,
          literal: self.ch.to_string(),
        },
        '*' => Token {
          token_type: TokenType::Asterisk,
          literal: self.ch.to_string(),
        },
        '<' => Token {
          token_type: TokenType::Lt,
          literal: self.ch.to_string(),
        },
        '>' => Token {
          token_type: TokenType::Gt,
          literal: self.ch.to_string(),
        },
        '{' => Token {
            token_type: TokenType::Lbrace,
            literal: self.ch.to_string(),
        },
        '}' => Token {
            token_type: TokenType::Rbrace,
            literal: self.ch.to_string(),
        },
        '\0' => Token {
          token_type: TokenType::Eof,
          literal: String::from(""),
        }, 
        _ => {
            if is_letter(self.ch) {
                let literal = self.read_identifier();
                let token_type = Keywords::lookup_ident(literal.as_str());
                return Token {
                    token_type: token_type,
                    literal: literal,
                }
            } else if is_digit(self.ch) {
                return Token {
                    token_type: TokenType::Int,
                    literal: self.read_number(),
                }
            } else {
                Token {
                    token_type: TokenType::Illegal,
                    literal: self.ch.to_string(),
                }
            }
        },
    };
    self.read_char();
    tok
  }
}

#[cfg(test)]
mod tests {
  use crate::token::TokenType;
  use crate::lexer::Lexer;

  #[test]
  fn next_token() {
    let input = "()";
    let mut l = Lexer::new(input.to_string());
    let t = l.next_token();
    assert_eq!(TokenType::Lparen, t.token_type);
  }

  #[test]
  fn next_next_token() {
    let input = "()";
    let mut l = Lexer::new(input.to_string());
    let _t = l.next_token();
    let t = l.next_token();
    assert_eq!(TokenType::Rparen, t.token_type);  
  }

  #[test]
  fn test_next_token() {
    let input = "let five = 5;
    let ten = 10;
    
    let add = fn(x, y) {
      x + y;
    };
    
    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;

    if (5 < 10) {
        return true;
    } else {
        return false;
    }
    ";

    let test = vec![
      (TokenType::Let, "let"),
      (TokenType::Ident, "five"),
      (TokenType::Assign, "="),
      (TokenType::Int, "5"),
      (TokenType::SemiColon, ";"),
      (TokenType::Let, "let"),
      (TokenType::Ident, "ten"),
      (TokenType::Assign, "="),
      (TokenType::Int, "10"),
      (TokenType::SemiColon, ";"),
      (TokenType::Let, "let"),
      (TokenType::Ident, "add"),
      (TokenType::Assign, "="),
      (TokenType::Function, "fn"),
      (TokenType::Lparen, "("),
      (TokenType::Ident, "x"),
      (TokenType::Comma, ","),
      (TokenType::Ident, "y"),
      (TokenType::Rparen, ")"),
      (TokenType::Lbrace, "{"),
      (TokenType::Ident, "x"),
      (TokenType::Plus, "+"),
      (TokenType::Ident, "y"),
      (TokenType::SemiColon, ";"),
      (TokenType::Rbrace, "}"),
      (TokenType::SemiColon, ";"),
      (TokenType::Let, "let"),
      (TokenType::Ident, "result"),
      (TokenType::Assign, "="),
      (TokenType::Ident, "add"),
      (TokenType::Lparen, "("),
      (TokenType::Ident, "five"),
      (TokenType::Comma, ","),
      (TokenType::Ident, "ten"),
      (TokenType::Rparen, ")"),
      (TokenType::SemiColon, ";"),
      (TokenType::Bang, "!"),
      (TokenType::Minus, "-"),
      (TokenType::Slash, "/"),
      (TokenType::Asterisk, "*"),
      (TokenType::Int, "5"),
      (TokenType::SemiColon, ";"),
      (TokenType::Int, "5"),
      (TokenType::Lt, "<"),
      (TokenType::Int, "10"),
      (TokenType::Gt, ">"),
      (TokenType::Int, "5"),
      (TokenType::SemiColon, ";"),
      (TokenType::If, "if"),
      (TokenType::Lparen, "("),
      (TokenType::Int, "5"),
      (TokenType::Lt, "<"),
      (TokenType::Int, "10"),
      (TokenType::Rparen, ")"),
      (TokenType::Lbrace, "{"),
      (TokenType::Return, "return"),
      (TokenType::True, "true"),
      (TokenType::SemiColon, ";"),
      (TokenType::Rbrace, "}"),
      (TokenType::Else, "else"),
      (TokenType::Lbrace, "{"),
      (TokenType::Return, "return"),
      (TokenType::False, "false"),
      (TokenType::SemiColon, ";"),
      (TokenType::Rbrace, "}"),
      (TokenType::Eof, "")
    ];

    let mut lexer = Lexer::new(input.to_string());

    for (expected_type, expected_literal) in &test {
      let tok = lexer.next_token();
      assert_eq!(tok.token_type, *expected_type);      
      assert_eq!(tok.literal, expected_literal.to_string());
    }
  }
} 
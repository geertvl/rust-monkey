pub mod lexer;
pub mod token;
pub mod support;

use lexer::Lexer;
use crate::token::TokenType;

fn main() {
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
      (TokenType::Eof, "")
    ];

    let mut lexer = Lexer::new(input.to_string());
    let mut counter = 0;
    for (expected_type, expected_literal) in &test {
      let tok = lexer.next_token();
      println!("read: {}", tok.literal);
      let el: String = expected_literal.to_string();
      println!("counter: {}", counter);  
      assert_eq!(tok.token_type, *expected_type);    
      assert_eq!(tok.literal, el);
      counter = counter + 1;
    }
}

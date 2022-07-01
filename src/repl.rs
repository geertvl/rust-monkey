use crate::token::{Token, TokenType};
use crate::lexer::Lexer;
use std::io;
use std::io::Write;

const PROMPT: &str = ">> ";

pub fn start() -> io::Result<()> {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let mut l = Lexer::new(buffer);

        let mut tok: Token = l.next_token();
        println!("{:?}", tok);
        while tok.token_type != TokenType::Eof {
            tok = l.next_token();
            println!("{:?}", tok);
        }
    }
}
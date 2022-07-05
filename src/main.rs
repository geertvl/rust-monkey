pub mod lexer;
pub mod token;
pub mod support;
pub mod repl;
pub mod ast;
pub mod parser;

use repl::start;

fn main() {
    println!("Monkey Programming Language");
    start().unwrap();
}

pub mod lexer;
pub mod token;
pub mod support;
pub mod repl;

use repl::start;

fn main() {
    println!("Monkey Programming Language");
    start().unwrap();
}

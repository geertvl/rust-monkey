pub mod lexer;
pub mod token;
pub mod support;

use lexer::Lexer;

fn main() {
    let _l = Lexer::new(String::from("test"));
}

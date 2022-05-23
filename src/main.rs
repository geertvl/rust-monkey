pub mod lexer;
pub mod token;

use lexer::Lexer;

fn main() {
    let _l = Lexer::new(String::from("test"));
}

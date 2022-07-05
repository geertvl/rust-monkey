use crate::lexer::Lexer;
use crate::token::Token;
use crate::ast::Program;

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    fn new(lexer: Lexer) -> Parser {
        let mut p = Parser { lexer: lexer, current_token: None, peek_token: None };
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.current_token = match &self.peek_token {
            Some(pk) => Some(pk.clone()),
            None => None,
        };
        self.peek_token = Some(self.lexer.next_token());
    }

    fn parse_program(&self) -> Option<Program> {
        None
    }
}


#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::ast::Statement;

    #[test]
    fn test_let_statements() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;";

        let l = Lexer::new(input.to_string());
        let p = Parser::new(l);

        let program = match p.parse_program() {
            Some(p) => p,
            None => panic!("ParseProgram() failed"),
        };
        assert_eq!(program.statements.len(), 3);

        let tests = vec![
            "x",
            "y",
            "foobar",
        ];

        for (i, tt) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            test_let_statement(*stmt, tt);
        }
    }

    fn test_let_statement(stmt: Box<dyn Statement>, expected_identifier: &str) -> bool {
        let token_literal = stmt.token_literal();
        
    }
}
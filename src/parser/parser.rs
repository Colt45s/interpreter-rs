use super::ast;
use crate::lexer::lexer::Lexer;
use crate::lexer::token::Token;

struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token
}

impl <'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser {
        let mut parser = Parser{
            lexer,
            current_token: Token::Illegal,
            peek_token: Token::Illegal
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
          self.current_token = self.peek_token.clone();
          self.peek_token = self.lexer.next_token()  
    }

    fn expect_peek(&mut self, token: Token) -> Result<(), String> {
        match self.peek_token_is(&token) {
            true => {
                self.next_token();
                Ok(())
            },
            false => return Err(format!("Expect token {0}. But received {1}", token, self.peek_token))
        }
    }

    fn peek_token_is(&mut self, t: &Token)-> bool {
        self.peek_token == *t
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, String> {
        let mut program = ast::Program::default();

        while self.current_token != Token::EOF {
            let statement = self.parse_statement();
            match statement {
                Some(s) => {
                    program.statements.push(s);
                },
                None => {}
            }
            self.next_token();
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.current_token {
            Token::Let => {
                let statement = self.parse_let_statement().unwrap();
                Some(statement)
            },
            _ => return None
        }
    }

    fn parse_let_statement(&mut self) ->  Result<ast::Statement, String> {
        let name = self.parse_indent()?;

        self.expect_peek(Token::Assign)?;

        self.next_token();
        let literal = match self.current_token {
            Token::Int(l) =>  ast::Expression::Literal(ast::Literal::Int(l)),
            _ => return Err(format!("Invalid token {}", self.current_token))
        };

        while !self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Ok(ast::Statement::Let(name, literal))
    }

    fn parse_indent(&mut self) -> Result<ast::Identifier, String> {
        let ident = match &self.peek_token {
            Token::Ident(i) => ast::Identifier(i.to_string()),
            _ => return Err(format!("Invalid identifier {}", self.peek_token))
        };

        self.next_token();
        Ok(ident)
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    fn parse(input: &str) -> ast::Program {
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        parser.parse_program().unwrap()
    }

    #[test]
    fn test_let_statement() {
        let inputs = vec![
            ("let x = 5;", String::from("x"), ast::Literal::Int(5)),
            ("let y = 10;", String::from("y"), ast::Literal::Int(10)),
            ("let foobar = 838383;", String::from("foobar"), ast::Literal::Int(838383)),
        ];

        for (input, expect_ident_value, expect_literal_value) in inputs {
            let program = parse(input);
            assert!(program.statements.len() > 0);
            let target = program.statements.get(0);
            match target {
                Some(ast::Statement::Let(ident, exp)) => {
                    assert_eq!(*ident, ast::Identifier(expect_ident_value));
                    assert_eq!(*exp, ast::Expression::Literal(expect_literal_value));      
                },
                Some(_) => panic!(),
                None => panic!(),
            }
        }
    }
}
use crate::ast;
use crate::lexer::Lexer;
use crate::token::Token;

struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token 
}

impl <'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser {
        let mut parser = Parser{
            lexer,
            current_token: Token::ILLEGAL,
            peek_token: Token::ILLEGAL,
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
        match self.peek_token_is(token) {
            true => {
                self.next_token();
                Ok(())
            },
            false => return Err("err".to_string())
        }
    }

    fn peek_token_is(&mut self, t: Token)-> bool {
        self.peek_token == t
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program { statements: vec![] };

        while self.current_token != Token::EOF {
            match self.parse_statement() {
                Ok(statement) => program.statements.push(statement),
                Err(_msg) => {}
            }
            self.next_token()
        }

        program
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, String> {
        match self.current_token {
            Token::LET => self.parse_let_statement(),
            _ => Err("err".to_string())
        }
    }

    fn parse_let_statement(&mut self) ->  Result<ast::Statement, String> {
        self.expect_peek(Token::IDENT("_".to_string()))?;

        let name = match self.parse_indent() {
            Some(name) => name,
            _ => return Err("err".to_string())
        };

        self.next_token();
        let literal = match self.current_token {
            Token::INT(l) =>  ast::Expression::Literal(ast::Literal::Int(l)),
            _ => return Err("err".to_string())
        };

        self.expect_peek(Token::ASSIGN)?;

        while !self.peek_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Ok(ast::Statement::Let(name, literal))
    }

    fn parse_indent(&mut self) -> Option<ast::Identifier> {
        match &self.current_token {
            Token::IDENT(i) => Some(ast::Identifier(i.to_string())),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statement() {
        let i = "
		let x = 5;
		let y = 10;
		let foobar = 838383;
		";

        let input = i.to_string();

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        assert_eq!(
            vec![
                ast::Statement::Let(ast::Identifier(String::from("x")), ast::Expression::Literal(ast::Literal::Int(5))),
                ast::Statement::Let(ast::Identifier(String::from("y")), ast::Expression::Literal(ast::Literal::Int(10))),
                ast::Statement::Let(
                    ast::Identifier(String::from("foobar")),
                    ast::Expression::Literal(ast::Literal::Int(838383)),
                ),
            ],
            program.statements,
        );
    }
}
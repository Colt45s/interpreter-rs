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
            peek_token: Token::Illegal,
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

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program::default();

        while self.current_token != Token::EOF {
            match self.parse_statement() {
                Ok(statement) => program.statements.push(statement),
                Err(msg) => println!("{}", msg)
            }
            self.next_token()
        }

        program
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, String> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            _ => Err("err".to_string())
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
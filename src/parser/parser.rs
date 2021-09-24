use super::ast;
use crate::lexer::lexer::Lexer;
use crate::lexer::token::Token;
use thiserror::Error;

enum Priority {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}

struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    errors: Vec<ParserError>,
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("expect token (expected {expected:?}, found {found:?})")]
    ExpectToken { expected: String, found: String },
    #[error("expect expression token. {0}")]
    ExpectExpression(String),
    #[error("unable to parse integer. {0}")]
    UnableToParseInteger(String),
}

type Result<T> = std::result::Result<T, ParserError>;

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: Token::Illegal,
            peek_token: Token::Illegal,
            errors: vec![],
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token()
    }

    fn expect_peek(&mut self, token: Token) -> Result<()> {
        if self.peek_token_is(&token) {
            self.next_token();
            Ok(())
        } else {
            Err(ParserError::ExpectToken {
                expected: format!("{}", token),
                found: format!("{}", self.peek_token.clone()),
            })
        }
    }

    fn peek_token_is(&mut self, t: &Token) -> bool {
        match t {
            Token::Ident(_) => match self.peek_token {
                Token::Ident(_) => true,
                _ => false,
            },
            Token::Int(_) => match self.peek_token {
                Token::Int(_) => true,
                _ => false,
            },
            t => self.peek_token == *t,
        }
    }

    pub fn parse_program(&mut self) -> Result<ast::Program> {
        let mut program = ast::Program::default();

        while self.current_token != Token::EOF {
            if let Ok(statement) = self.parse_statement() {
                program.statements.push(statement);
            }

            match self.parse_statement() {
                Ok(statement) => program.statements.push(statement),
                Err(e) => {
                    self.errors.push(e);
                }
            }
            self.next_token();
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<ast::Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<ast::Statement> {
        self.expect_peek(Token::Ident("_".to_string()))?;

        let name = ast::Identifier(match &self.current_token {
            Token::Ident(ident) => ident.clone(),
            _ => unreachable!(),
        });

        self.expect_peek(Token::Assign)?;

        self.next_token();
        let literal = self.parse_expression(Priority::Lowest)?;
        while !self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Ok(ast::Statement::Let(name, literal))
    }

    fn parse_return_statement(&mut self) -> Result<ast::Statement> {
        self.next_token();
        let literal = self.parse_expression(Priority::Lowest)?;
        while !self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Ok(ast::Statement::Return(literal))
    }

    fn parse_expression_statement(&mut self) -> Result<ast::Statement> {
        let expr = self.parse_expression(Priority::Lowest)?;

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Ok(ast::Statement::Expr(expr))
    }

    fn parse_expression(&mut self, priority: Priority) -> Result<ast::Expression> {
        let prefix = match &self.current_token {
            Token::Ident(_) => self.parse_identifier()?,
            Token::Int(_) => self.parse_integer_literal()?,
            t => return Err(ParserError::ExpectExpression(format!("{}", t))),
        };
        Ok(prefix.clone())
    }

    fn parse_identifier(&mut self) -> Result<ast::Expression> {
        let val = match &self.current_token {
            Token::Ident(ident) => ident.clone(),
            t => Err(ParserError::ExpectToken {
                expected: "Ident".to_string(),
                found: format!("{}", t),
            })?,
        };

        Ok(ast::Expression::Ident(ast::Identifier(val)))
    }

    fn parse_integer_literal(&mut self) -> Result<ast::Expression> {
        let parsed = match &self.current_token {
            Token::Int(val) => val.clone().parse::<i32>().map_err(|_| {
                ParserError::UnableToParseInteger(format!("{}", self.current_token))
            })?,
            t => Err(ParserError::ExpectToken {
                expected: "Int".to_string(),
                found: format!("{}", t),
            })?,
        };
        Ok(ast::Expression::IntegerLiteral(parsed))
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
            (
                "let x = 5;",
                String::from("x"),
                ast::Expression::IntegerLiteral(5),
            ),
            (
                "let y = 10;",
                String::from("y"),
                ast::Expression::IntegerLiteral(10),
            ),
            (
                "let foobar = 838383;",
                String::from("foobar"),
                ast::Expression::IntegerLiteral(838383),
            ),
        ];

        for (input, expect_ident_value, expect_literal_value) in inputs {
            let program = parse(input);
            assert!(program.statements.len() > 0);
            let target = program.statements.get(0);
            if let Some(ast::Statement::Let(ident, exp)) = target {
                assert_eq!(*ident, ast::Identifier(expect_ident_value));
                assert_eq!(*exp, expect_literal_value);
            } else {
                panic!();
            }
        }
    }

    #[test]
    fn test_return_statement() {
        let inputs = vec![
            ("return 5;", ast::Expression::IntegerLiteral(5)),
            ("return 10;", ast::Expression::IntegerLiteral(10)),
            ("return 993322;", ast::Expression::IntegerLiteral(993322)),
        ];

        for (input, expect_literal_value) in inputs {
            let program = parse(input);
            assert!(program.statements.len() > 0);
            let target = program.statements.get(0);
            if let Some(ast::Statement::Return(exp)) = target {
                assert_eq!(*exp, expect_literal_value);
            } else {
                panic!();
            }
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = ("foobar;", "foobar".to_string());
        let program = parse(input.0);
        assert!(program.statements.len() > 0);
        let target = program.statements.get(0);
        if let Some(ast::Statement::Expr(ast::Expression::Ident(identifier))) = target {
            assert_eq!(*identifier, ast::Identifier(input.1));
        } else {
            panic!();
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = ("5;", 5);
        let program = parse(input.0);
        assert!(program.statements.len() > 0);
        let target = program.statements.get(0);
        if let Some(ast::Statement::Expr(ast::Expression::IntegerLiteral(int))) = target {
            assert_eq!(*int, input.1);
        } else {
            panic!();
        }
    }
}

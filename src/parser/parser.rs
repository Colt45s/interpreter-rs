use super::ast;
use super::precedence::Precedence;
use crate::lexer::Lexer;
use crate::lexer::Token;
use thiserror::Error;

struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    errors: ParserErrors,
}

#[derive(Error, Debug, Clone)]
#[error("{}", (.0).iter().map(|e| format!("\x32{}", e)).collect::<Vec<_>>().join("\n"))]
pub struct ParserErrors(Vec<ParserError>);

#[derive(Error, Debug, Clone)]
pub enum ParserError {
    #[error("expect token (expected {expected:?}, found {found:?})")]
    ExpectToken { expected: String, found: String },
    #[error("expect expression token. {0}")]
    ExpectExpression(String),
    #[error("unable to parse integer. {0}")]
    UnableToParseInteger(String),
    #[error("unable to parse operator. {0}")]
    UnableToParseOperator(String),
}

type ParserResult<T> = std::result::Result<T, ParserErrors>;
type Result<T> = std::result::Result<T, ParserError>;

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: Token::Illegal,
            peek_token: Token::Illegal,
            errors: ParserErrors(Vec::new()),
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

    pub fn parse_program(&mut self) -> ParserResult<ast::Program> {
        let mut program = ast::Program::default();

        while self.current_token != Token::EOF {
            match self.parse_statement() {
                Ok(statement) => program.statements.push(statement),
                Err(e) => {
                    self.errors.0.push(e);
                }
            }
            self.next_token();
        }

        if !self.errors.0.is_empty() {
            return Err(self.errors.clone());
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
        let literal = self.parse_expression(Precedence::Lowest)?;
        while !self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Ok(ast::Statement::Let(name, literal))
    }

    fn parse_return_statement(&mut self) -> Result<ast::Statement> {
        self.next_token();
        let literal = self.parse_expression(Precedence::Lowest)?;
        while !self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Ok(ast::Statement::Return(literal))
    }

    fn parse_expression_statement(&mut self) -> Result<ast::Statement> {
        let expr = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Ok(ast::Statement::Expr(expr))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<ast::Expression> {
        let mut left = match &self.current_token {
            Token::Ident(_) => self.parse_identifier()?,
            Token::Int(_) => self.parse_integer_literal()?,
            Token::Bang | Token::Minus => self.parse_prefix_expression()?,
            t => return Err(ParserError::ExpectExpression(format!("{}", t))),
        };

        while !self.peek_token_is(&Token::Semicolon)
            && precedence < Precedence::from(&self.peek_token)
        {
            match &self.peek_token {
                Token::Plus
                | Token::Minus
                | Token::Slash
                | Token::Asterisk
                | Token::Eq
                | Token::Neq
                | Token::Lt
                | Token::Gt => {
                    self.next_token();
                    left = self.parse_infix_expression(Box::new(left))?;
                }
                _ => break,
            };
        }
        Ok(left)
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

    fn parse_prefix_expression(&mut self) -> Result<ast::Expression> {
        let token = self.current_token.clone();
        let operator = parse_to_operator(&self.current_token)?;
        self.next_token();
        let right = Box::new(self.parse_expression(Precedence::Prefix)?);
        Ok(ast::Expression::Prefix {
            token,
            operator,
            right,
        })
    }

    fn parse_infix_expression(&mut self, left: Box<ast::Expression>) -> Result<ast::Expression> {
        let token = self.current_token.clone();
        let operator = parse_to_operator(&self.current_token)?;
        let precedence = Precedence::from(&self.current_token);
        self.next_token();
        let right = Box::new(self.parse_expression(precedence)?);
        Ok(ast::Expression::Infix {
            token,
            operator,
            left,
            right,
        })
    }
}

fn parse_to_operator(token: &Token) -> Result<ast::Operator> {
    Ok(match token {
        Token::Assign => ast::Operator::Assign,
        Token::Plus => ast::Operator::Plus,
        Token::Minus => ast::Operator::Minus,
        Token::Bang => ast::Operator::Bang,
        Token::Asterisk => ast::Operator::Asterisk,
        Token::Slash => ast::Operator::Slash,
        Token::Lt => ast::Operator::Lt,
        Token::Gt => ast::Operator::Gt,
        Token::Eq => ast::Operator::Eq,
        Token::Neq => ast::Operator::Neq,
        _ => return Err(ParserError::UnableToParseOperator(format!("{}", token))),
    })
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    fn parse(input: &str) -> super::ParserResult<ast::Program> {
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        parser.parse_program()
    }

    #[test]
    fn test_let_statement() {
        let inputs = vec![
            ("let x = 5;", String::from("x"), 5),
            ("let y = 10;", String::from("y"), 10),
            ("let foobar = 838383;", String::from("foobar"), 838383),
        ];

        for (input, expect_ident_value, expect_literal_value) in inputs {
            match parse(input) {
                Err(errors) => test_print_errors(errors),
                Ok(program) => {
                    assert!(program.statements.len() > 0);
                    let target = program.statements.get(0);
                    if let Some(ast::Statement::Let(ident, exp)) = target {
                        test_identifier(ident, expect_ident_value);
                        test_integer_literal(exp, expect_literal_value);
                    } else {
                        panic!();
                    }
                }
            };
        }
    }

    #[test]
    fn test_return_statement() {
        let inputs = vec![
            ("return 5;", 5),
            ("return 10;", 10),
            ("return 993322;", 993322),
        ];

        for (input, expect_literal_value) in inputs {
            match parse(input) {
                Err(errors) => test_print_errors(errors),
                Ok(program) => {
                    assert!(program.statements.len() > 0);
                    let target = program.statements.get(0);
                    if let Some(ast::Statement::Return(exp)) = target {
                        test_integer_literal(exp, expect_literal_value);
                    } else {
                        panic!();
                    }
                }
            };
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = ("foobar;", "foobar".to_string());
        match parse(input.0) {
            Err(errors) => test_print_errors(errors),
            Ok(program) => {
                assert!(program.statements.len() > 0);
                let target = program.statements.get(0);
                if let Some(ast::Statement::Expr(ast::Expression::Ident(identifier))) = target {
                    test_identifier(identifier, input.1);
                } else {
                    panic!();
                }
            }
        };
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = ("5;", 5);
        match parse(input.0) {
            Err(errors) => test_print_errors(errors),
            Ok(program) => {
                assert!(program.statements.len() > 0);
                let target = program.statements.get(0);
                if let Some(ast::Statement::Expr(expression)) = target {
                    test_integer_literal(&expression, input.1);
                } else {
                    panic!();
                }
            }
        };
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let inputs = vec![("!5;", "!", 5), ("-15;", "-", 15)];

        for (input, prefix, expect_right_value) in inputs {
            match parse(input) {
                Err(errors) => test_print_errors(errors),
                Ok(program) => {
                    assert!(program.statements.len() > 0);
                    let target = program.statements.get(0);
                    if let Some(ast::Statement::Expr(ast::Expression::Prefix {
                        operator,
                        right,
                        ..
                    })) = target
                    {
                        test_operator(operator, prefix);
                        test_integer_literal(&*right, expect_right_value);
                    } else {
                        panic!();
                    }
                }
            };
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let inputs = vec![
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];

        for (input, expect_left_value, infix, expect_right_value) in inputs {
            match parse(input) {
                Err(errors) => test_print_errors(errors),
                Ok(program) => {
                    assert!(program.statements.len() > 0);
                    let target = program.statements.get(0);
                    if let Some(ast::Statement::Expr(ast::Expression::Infix {
                        left,
                        operator,
                        right,
                        ..
                    })) = target
                    {
                        test_integer_literal(&*left, expect_left_value);
                        test_operator(operator, infix);
                        test_integer_literal(&*right, expect_right_value);
                    } else {
                        panic!();
                    }
                }
            };
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let inputs = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
        ];

        inputs
            .iter()
            .for_each(|(input, expected)| match parse(input) {
                Err(errors) => test_print_errors(errors),
                Ok(program) => {
                    assert_eq!(&format!("{}", program), expected);
                }
            });
    }

    fn test_identifier(identifier: &ast::Identifier, v: String) {
        assert_eq!(*identifier, ast::Identifier(v));
    }

    fn test_integer_literal(il: &ast::Expression, v: i32) {
        if let ast::Expression::IntegerLiteral(int) = il {
            assert_eq!(int, &v);
        } else {
            panic!();
        }
    }

    fn test_operator(operator: &ast::Operator, expect_operator: &str) {
        assert_eq!(operator.to_string(), expect_operator,);
    }

    fn test_print_errors(errors: ParserErrors) {
        errors.0.iter().for_each(|e| {
            println!("error! {}", e);
        });
    }
}

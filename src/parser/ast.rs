use crate::lexer::token::Token;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.statements
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(Identifier, Expression),
    Return(Expression),
    Expr(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Let(identifier, expression) => {
                write!(f, "let {} = {};", identifier, expression)
            }
            Statement::Return(expression) => write!(f, "return {};", expression),
            Statement::Expr(expression) => write!(f, "{}", expression),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier(pub String);

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Identifier(ident) = self;
        write!(f, "{}", &ident)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Ident(Identifier),
    IntegerLiteral(i32),
    Prefix {
        token: Token,
        operator: Operator,
        right: Box<Expression>,
    },
    Infix {
        token: Token,
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Ident(indentifier) => write!(f, "{}", indentifier),
            Expression::IntegerLiteral(val) => write!(f, "{}", val),
            Expression::Prefix {
                operator, right, ..
            } => write!(f, "({}{})", operator, right),
            Expression::Infix {
                left,
                operator,
                right,
                ..
            } => write!(f, "({} {} {})", left, operator, right),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    Neq,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self {
            Operator::Assign => "=",
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Bang => "!",
            Operator::Asterisk => "*",
            Operator::Slash => "/",
            Operator::Lt => "<",
            Operator::Gt => ">",
            Operator::Eq => "==",
            Operator::Neq => "!=",
        };

        write!(f, "{}", val)
    }
}

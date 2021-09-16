use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    EOF,
    Ident(String),
    Int(i32),
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
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Ident(name) => write!(f, "{}", name),
            Token::Int(val) => write!(f, "{}", val),
            Token::Minus => write!(f, "-"),
            Token::Plus => write!(f, "+"),
            Token::Bang => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Gt => write!(f, ">"),
            Token::Lt => write!(f, "<"),
            Token::Eq => write!(f, "=="),
            Token::Neq => write!(f, "!="),
            Token::Semicolon => write!(f, ";"),
            Token::Assign => write!(f, "="),
            Token::Function => write!(f, "fn"),
            Token::Lparen => write!(f, "("),
            Token::Rparen => write!(f, ")"),
            Token::Lbrace => write!(f, "{{"),
            Token::Rbrace => write!(f, "}}"),
            Token::Comma => write!(f, ","),
            tok => write!(f, "{:?}", tok),
        }
    }
}

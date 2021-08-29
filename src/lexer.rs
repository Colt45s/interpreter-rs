use std::str;

use crate::token::Token;

struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            b'=' => Token::ASSIGN,
            b';' => Token::SEMICOLON,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b',' => Token::COMMA,
            b'+' => Token::PLUS,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            0 => Token::EOF,
            _ => Token::ILLEGAL,
        };

        self.read_char();
        tok
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
        let ten = 10;
        let add = fn(x, y) {
          x + y;
        };

        let result = add(five, ten);
        "#;
        let tests = vec![
            Token::LET,
            Token::IDENT,
            Token::ASSIGN,
            Token::INT,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT,
            Token::ASSIGN,
            Token::INT,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT,
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT,
            Token::COMMA,
            Token::IDENT,
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT,
            Token::PLUS,
            Token::IDENT,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT,
            Token::ASSIGN,
            Token::IDENT,
            Token::LPAREN,
            Token::IDENT,
            Token::LPAREN,
            Token::IDENT,
            Token::COMMA,
            Token::IDENT,
            Token::RPAREN,
            Token::SEMICOLON,
            Token::EOF,
        ];
        let mut lexer = Lexer::new(input);
        for t in tests {
            let tok = lexer.next_token();
            assert_eq!(t, tok);
        }
    }
}

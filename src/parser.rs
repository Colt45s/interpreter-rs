use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::Token;

struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    peek_token: Token 
}

impl <'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser{
            lexer,
            cur_token: Token::ILLEGAL,
            peek_token: Token::ILLEGAL,
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
          self.cur_token = self.peek_token.clone();
          self.peek_token = self.lexer.next_token()  
    }
}
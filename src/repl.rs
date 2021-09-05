use std::io;

use crate::{lexer::Lexer, token::Token};

pub fn start() {
    let mut input = String::new();

    loop {
        print!(">> ");
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);
            }
            Err(error) => {
                println!("error: {}", error);
                return;
            }
        }

        let mut lexer = Lexer::new(&mut input);

        loop {
            let tok = lexer.next_token();
            println!("{:?}", tok);
            if tok == Token::EOF {
                break;
            }
        }
    }
}

use crate::lexer::{Lexer, Token};
use std::io;

pub fn start() {
    let mut input = String::new();

    loop {
        print!(">> ");
        io::Write::flush(&mut io::stdout()).expect("Flush failed");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

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

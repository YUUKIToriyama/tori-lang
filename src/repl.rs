use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::io::Write;

pub struct Repl {
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
}

impl Repl {
    pub fn new(stdin: std::io::Stdin, stdout: std::io::Stdout) -> Repl {
        Repl { stdin, stdout }
    }

    pub fn start(&mut self) {
        const PROMPT: &str = ">> ";
        print!("This is the torilang interactive interface!\n{}", PROMPT);
        self.stdout.flush().unwrap();

        let mut input = String::new();
        while self.stdin.read_line(&mut input).is_ok() {
            let mut lexer = Lexer::new(&input);
            let mut tokens: Vec<Token> = vec![];
            let mut token = lexer.get_next_token();
            while token.token_type != TokenType::EOF {
                tokens.push(token);
                token = lexer.get_next_token();
            }
            tokens.push(token);
            print!("{:?}\n{}", tokens, PROMPT);
            self.stdout.flush().unwrap();
            input.clear();
        }
    }
}

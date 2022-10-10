use crate::lexer::Lexer;
use crate::token::Token;
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
            while lexer.has_next() {
                tokens.push(lexer.get_next_token());
                lexer.read_next();
            }
            print!("{:?}\n{}", tokens, PROMPT);
            self.stdout.flush().unwrap();
            input.clear();
        }
    }
}

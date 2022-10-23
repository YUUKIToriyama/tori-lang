use crate::{
    ast::{LetStatement, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    next_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let current_token = lexer.get_next_token();
        let next_token = lexer.get_next_token();
        Parser {
            lexer,
            current_token,
            next_token,
        }
    }

    pub fn read_next(&mut self) -> Result<(), &str> {
        if self.next_token.token_type == TokenType::EOF {
            Err("This is the end of program")
        } else {
            self.current_token = self.next_token.clone();
            self.next_token = self.lexer.get_next_token();
            Ok(())
        }
    }
}

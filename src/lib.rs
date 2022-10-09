mod token;

use crate::token::{Token, TokenType};

struct Lexer {
    input: String,
    position: usize,
    position_next: usize,
    current_character: char,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        let current = input.chars().nth(0).unwrap();
        Lexer {
            input: input.to_string(),
            position: 0,
            position_next: 1,
            current_character: current,
        }
    }

    fn read_next(&mut self) {
        if self.position_next < self.input.len() {
            self.current_character = self.input.chars().nth(self.position_next).unwrap();
        }
        self.position = self.position_next;
        self.position_next = self.position_next + 1;
    }

    fn get_next_token(&mut self) -> Token {
        let token = match self.current_character {
            '=' => Token::new(TokenType::ASSIGN, '='),
            ';' => Token::new(TokenType::SEMICOLON, ';'),
            '(' => Token::new(TokenType::LPAREN, '('),
            ')' => Token::new(TokenType::RPAREN, ')'),
            ',' => Token::new(TokenType::COMMA, ','),
            '+' => Token::new(TokenType::PLUS, '+'),
            '{' => Token::new(TokenType::LBRACE, '{'),
            '}' => Token::new(TokenType::RBRACE, '}'),
            _ => Token::new(TokenType::EOF, '\0'),
        };
        self.read_next();
        token
    }
}

#[cfg(test)]
mod tests_for_lexer {
    use crate::{Lexer, TokenType};

    #[test]
    fn test_new() {
        let lexer = Lexer::new("=;(),+{}");
        assert_eq!(lexer.position, 0);
        assert_eq!(lexer.position_next, 1);
        assert_eq!(lexer.current_character, '=');
    }

    #[test]
    fn test_get_next_token() {
        let mut lexer = Lexer::new("=;(),+{}");
        let mut token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::ASSIGN);
        token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::SEMICOLON);
    }
}

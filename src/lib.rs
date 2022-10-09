mod token;

use crate::token::{Token, TokenType};

fn is_letter(character: char) -> bool {
    character.is_ascii_alphabetic() || character == '_'
}

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
            '=' => Token::new(TokenType::ASSIGN, "=".to_string()),
            ';' => Token::new(TokenType::SEMICOLON, ";".to_string()),
            '(' => Token::new(TokenType::LPAREN, "(".to_string()),
            ')' => Token::new(TokenType::RPAREN, ")".to_string()),
            ',' => Token::new(TokenType::COMMA, ",".to_string()),
            '+' => Token::new(TokenType::PLUS, "+".to_string()),
            '{' => Token::new(TokenType::LBRACE, "{".to_string()),
            '}' => Token::new(TokenType::RBRACE, "}".to_string()),
            _ => {
                let mut pending_characters: Vec<char> = vec![];
                while is_letter(self.current_character) {
                    pending_characters.push(self.current_character);
                    self.read_next();
                }
                Token::new(TokenType::IDENT, pending_characters.into_iter().collect())
            }
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

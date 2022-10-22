use crate::token::{Token, TokenType};

fn is_letter(character: char) -> bool {
    character.is_ascii_alphabetic() || character == '_'
}

fn is_digit(character: char) -> bool {
    character.is_ascii_digit()
}

fn lookup_identifier_type(literal: &str) -> TokenType {
    match literal {
        "let" => TokenType::LET,
        "fn" => TokenType::FUNCTION,
        "true" => TokenType::TRUE,
        "false" => TokenType::FALSE,
        "if" => TokenType::IF,
        "else" => TokenType::ELSE,
        "return" => TokenType::RETURN,
        _ => TokenType::IDENT,
    }
}

pub struct Lexer {
    input: String,
    position: usize,
    current_character: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let current = input.chars().nth(0).unwrap();
        Lexer {
            input: input.to_string(),
            position: 0,
            current_character: current,
        }
    }

    pub fn has_next(&self) -> bool {
        self.position + 1 < self.input.len()
    }

    pub fn read_next(&mut self) {
        if self.has_next() {
            self.current_character = self.input.chars().nth(self.position + 1).unwrap();
        }
        self.position = self.position + 1;
    }

    fn peek_next(&self) -> Option<char> {
        if self.has_next() {
            let next_character = self.input.chars().nth(self.position + 1).unwrap();
            Some(next_character)
        } else {
            None
        }
    }

    pub fn get_next_token(&mut self) -> Token {
        while self.current_character.is_whitespace() {
            self.read_next()
        }
        let token = match self.current_character {
            '=' => match self.peek_next() {
                Some('=') => {
                    self.read_next();
                    Token::new(TokenType::EQ, "==".to_string())
                }
                Some(_) => Token::new(TokenType::ASSIGN, "=".to_string()),
                None => Token::new(TokenType::EOF, "".to_string()),
            },
            '+' => Token::new(TokenType::PLUS, "+".to_string()),
            '-' => Token::new(TokenType::MINUS, "-".to_string()),
            '!' => match self.peek_next() {
                Some('=') => {
                    self.read_next();
                    Token::new(TokenType::NOTEQ, "!=".to_string())
                }
                Some(_) => Token::new(TokenType::BANG, "!".to_string()),
                None => Token::new(TokenType::EOF, "".to_string()),
            },
            '*' => Token::new(TokenType::ASTERISK, "*".to_string()),
            '/' => Token::new(TokenType::SLASH, "/".to_string()),
            '<' => Token::new(TokenType::LT, "<".to_string()),
            '>' => Token::new(TokenType::GT, ">".to_string()),
            ';' => Token::new(TokenType::SEMICOLON, ";".to_string()),
            '(' => Token::new(TokenType::LPAREN, "(".to_string()),
            ')' => Token::new(TokenType::RPAREN, ")".to_string()),
            ',' => Token::new(TokenType::COMMA, ",".to_string()),
            '{' => Token::new(TokenType::LBRACE, "{".to_string()),
            '}' => Token::new(TokenType::RBRACE, "}".to_string()),
            other => {
                if is_letter(other) {
                    let mut pending: Vec<char> = vec![];
                    while is_letter(self.current_character) {
                        pending.push(self.current_character);
                        match self.peek_next() {
                            None => break,
                            Some(v) => {
                                if is_letter(v) {
                                    self.read_next();
                                    continue;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    let literal: String = pending.iter().collect();
                    let token_type: TokenType = lookup_identifier_type(&literal);
                    Token::new(token_type, literal)
                } else if is_digit(other) {
                    let mut pending: Vec<char> = vec![];
                    while is_digit(self.current_character) {
                        pending.push(self.current_character);
                        match self.peek_next() {
                            None => break,
                            Some(v) => {
                                if is_digit(v) {
                                    self.read_next();
                                    continue;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    let literal: String = pending.iter().collect();
                    Token::new(TokenType::INT, literal)
                } else {
                    Token::new(TokenType::ILLEGAL, other.to_string())
                }
            }
        };
        self.read_next();
        token
    }
}

#[cfg(test)]
mod tests_for_lexer {
    use crate::lexer::{Lexer, TokenType};

    #[test]
    fn test_new() {
        let lexer = Lexer::new("=;(),+{}");
        assert_eq!(lexer.position, 0);
        assert_eq!(lexer.current_character, '=');
    }

    #[test]
    fn test_get_next_token() {
        let mut lexer = Lexer::new("let x = 12;");
        let mut token = lexer.get_next_token();
        assert_eq!(token.literal, "let");
        assert_eq!(token.token_type, TokenType::LET);
        token = lexer.get_next_token();
        assert_eq!(token.literal, "x");
        assert_eq!(token.token_type, TokenType::IDENT);
        token = lexer.get_next_token();
        assert_eq!(token.literal, "=");
        assert_eq!(token.token_type, TokenType::ASSIGN);
        token = lexer.get_next_token();
        assert_eq!(token.literal, "12");
        assert_eq!(token.token_type, TokenType::INT);
        token = lexer.get_next_token();
        assert_eq!(token.literal, ";");
        assert_eq!(token.token_type, TokenType::SEMICOLON);
    }

    #[test]
    fn test_get_next_token_2() {
        let mut lexer = Lexer::new("!(10 + 23 > 10 * 2 - 4)1");
        let mut token = lexer.get_next_token();
        assert_eq!(token.literal, "!");
        assert_eq!(token.token_type, TokenType::BANG);
        token = lexer.get_next_token();
        assert_eq!(token.literal, "(");
        assert_eq!(token.token_type, TokenType::LPAREN);
        token = lexer.get_next_token();
        assert_eq!(token.literal, "10");
        assert_eq!(token.token_type, TokenType::INT);
        token = lexer.get_next_token();
        assert_eq!(token.literal, "+");
        assert_eq!(token.token_type, TokenType::PLUS);
        token = lexer.get_next_token();
        assert_eq!(token.literal, "23");
        assert_eq!(token.token_type, TokenType::INT);
        token = lexer.get_next_token();
        assert_eq!(token.literal, ">");
        assert_eq!(token.token_type, TokenType::GT);
        token = lexer.get_next_token();
        assert_eq!(token.literal, "10");
        assert_eq!(token.token_type, TokenType::INT);
        token = lexer.get_next_token();
        assert_eq!(token.literal, "*");
        assert_eq!(token.token_type, TokenType::ASTERISK);
        token = lexer.get_next_token();
        assert_eq!(token.literal, "2");
        assert_eq!(token.token_type, TokenType::INT);
        token = lexer.get_next_token();
        assert_eq!(token.literal, "-");
        assert_eq!(token.token_type, TokenType::MINUS);
        token = lexer.get_next_token();
        assert_eq!(token.literal, "4");
        assert_eq!(token.token_type, TokenType::INT);
        token = lexer.get_next_token();
        assert_eq!(token.literal, ")");
        assert_eq!(token.token_type, TokenType::RPAREN);
    }

    #[test]
    fn test_get_next_token_3() {
        let code = "
            if (10 > 3) {
                return true;
            } else { 
                return false;
            }
        ";
        let mut lexer = Lexer::new(code);
        let mut token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::IF);
        token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::LPAREN);
        token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::INT);
        token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::GT);
        token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::INT);
        token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::RPAREN);
        token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::LBRACE);
        token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::RETURN);
        token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::TRUE);
        token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::SEMICOLON);
        token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::RBRACE);
        token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::ELSE);
        token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::LBRACE);
        token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::RETURN);
        token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::FALSE);
        token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::SEMICOLON);
        token = lexer.get_next_token();
        assert_eq!(token.token_type, TokenType::RBRACE);
    }

    #[test]
    fn test_2文字トークン_等価演算子() {
        let mut lexer = Lexer::new("a == 3;");
        let mut token = lexer.get_next_token();
        assert_eq!(token.literal, "a");
        assert_eq!(token.token_type, TokenType::IDENT);
        token = lexer.get_next_token();
        assert_eq!(token.literal, "==");
        assert_eq!(token.token_type, TokenType::EQ);
        token = lexer.get_next_token();
        assert_eq!(token.literal, "3");
        assert_eq!(token.token_type, TokenType::INT);
    }

    #[test]
    fn test_2文字トークン_非等価演算子() {
        let mut lexer = Lexer::new("a != 3;");
        let mut token = lexer.get_next_token();
        assert_eq!(token.literal, "a");
        assert_eq!(token.token_type, TokenType::IDENT);
        token = lexer.get_next_token();
        assert_eq!(token.literal, "!=");
        assert_eq!(token.token_type, TokenType::NOTEQ);
        token = lexer.get_next_token();
        assert_eq!(token.literal, "3");
        assert_eq!(token.token_type, TokenType::INT);
    }
}

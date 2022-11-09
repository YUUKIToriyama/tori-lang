use crate::ast::{Expression, Identifier};
use crate::{
    ast::LetStatement,
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

    fn parse_let_statement(&mut self) -> Result<LetStatement, &str> {
        if self.next_token.token_type != TokenType::IDENT {
            return Err("letキーワードの直後に識別子がありません");
        }
        let identifier = Identifier {
            label: self.next_token.literal.clone(),
        };
        self.read_next().unwrap();
        if self.next_token.token_type != TokenType::ASSIGN {
            return Err("識別子の直後に=がありません");
        }
        let mut tokens: Vec<Token> = vec![];
        loop {
            self.read_next().unwrap();
            tokens.push(self.next_token.clone());
            if self.next_token.token_type == TokenType::SEMICOLON {
                break;
            }
        }
        let expression = Expression { tokens: tokens };
        Ok(LetStatement {
            identifier: identifier,
            expression: expression,
        })
    }
}

#[cfg(test)]
mod parser_test {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::token::{Token, TokenType};

    #[test]
    fn let_statement_test() {
        let lexer = Lexer::new("let x = 10;");
        let mut parser = Parser::new(lexer);
        let statement = parser.parse_let_statement().unwrap();
        assert_eq!(statement.identifier.label, "x");
        assert_eq!(
            statement.expression.tokens[0],
            Token {
                token_type: TokenType::INT,
                literal: "10".to_string()
            }
        )
    }
}

use crate::ast::{Expression, Identifier, IntegerLiteral, ReturnStatement, Statement};

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

    fn parse_statement(&mut self) -> Statement {
        match self.current_token.token_type {
            TokenType::LET => Statement::LetStatement(self.parse_let_statement().unwrap()),
            TokenType::RETURN => Statement::ReturnStatement(self.parse_return_statement().unwrap()),
            _ => todo!(),
        }
    }

    fn parse_integer_literal(&self) -> IntegerLiteral {
        let literal = self.current_token.literal.clone();
        let mut num: u32 = 0;
        for i in 0..literal.len() {
            let exponent = (literal.len() - i - 1) as u32;
            let base: i32 = 10;
            let digit: u32 = match literal.chars().nth(i).unwrap() {
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => 0,
            };
            num = num + digit * base.pow(exponent) as u32;
        }
        IntegerLiteral {
            token_type: TokenType::INT,
            value: num,
        }
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement, &str> {
        if self.next_token.token_type != TokenType::IDENT {
            return Err("letキーワードの直後に識別子がありません");
        }
        let identifier = Identifier {
            token_type: TokenType::IDENT,
            value: self.next_token.literal.clone(),
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
        let expression = self.parse_integer_literal();
        Ok(LetStatement {
            token_type: TokenType::LET,
            identifier,
            expression: Expression::IntegerLiteral(expression),
        })
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement, &str> {
        if self.next_token.token_type == TokenType::SEMICOLON {
            return Err("返り値を指定してください");
        }
        loop {
            if self.next_token.token_type != TokenType::SEMICOLON {
                self.read_next().unwrap();
            } else {
                break;
            }
        }
        let expression = self.parse_integer_literal();
        Ok(ReturnStatement {
            token_type: TokenType::RETURN,
            expression: Expression::IntegerLiteral(expression),
        })
    }
}

#[cfg(test)]
mod parser_test {
    use crate::ast::{Expression, IntegerLiteral};
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::token::TokenType;

    #[test]
    fn let_statement_test() {
        let lexer = Lexer::new("let x = 10;");
        let mut parser = Parser::new(lexer);
        let statement = parser.parse_let_statement().unwrap();
        assert_eq!(statement.token_type, TokenType::LET);
        assert_eq!(statement.identifier.value, "x".to_string());
        assert_eq!(
            statement.expression,
            Expression::IntegerLiteral(IntegerLiteral {
                token_type: TokenType::INT,
                value: 10
            })
        );
    }

    #[test]
    fn return_statement_test() {
        let lexer = Lexer::new("return 10;");
        let mut parser = Parser::new(lexer);
        let statement = parser.parse_return_statement().unwrap();
        assert_eq!(statement.token_type, TokenType::RETURN);
        assert_eq!(
            statement.expression,
            Expression::IntegerLiteral(IntegerLiteral {
                token_type: TokenType::INT,
                value: 10
            })
        );
    }

    #[test]
    fn parse_integer_literal() {
        let lexer = Lexer::new("123");
        let parser = Parser::new(lexer);
        let literal = parser.parse_integer_literal();
        assert_eq!(literal.token_type, TokenType::INT);
        assert_eq!(literal.value, 123);
    }
}

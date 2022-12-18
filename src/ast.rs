use crate::token::TokenType;

/// プログラム
pub struct Program {
    pub statements: Vec<Statement>,
}

/// 文
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
}

// let文
pub struct LetStatement {
    pub token_type: TokenType,
    pub identifier: Identifier,
    pub expression: Expression,
}

// Return文
pub struct ReturnStatement {
    pub token_type: TokenType,
    pub expression: Expression,
}

/// 式
#[derive(Debug, PartialEq)]
pub enum Expression {
    IntegerLiteral(IntegerLiteral),
}

/// 整数リテラル
#[derive(Debug, PartialEq)]
pub struct IntegerLiteral {
    pub token_type: TokenType,
    pub value: u32,
}

/// 式文
pub struct ExpressionStatement {
    pub expression: Expression,
}

/// 識別子
pub struct Identifier {
    pub token_type: TokenType,
    pub value: String,
}

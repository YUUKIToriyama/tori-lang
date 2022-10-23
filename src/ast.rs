use crate::token::Token;

pub trait Statement {}

// let文
pub struct LetStatement {
    pub identifier: Identifier,
    pub expression: Expression,
}
impl Statement for LetStatement {}

/// 識別子
pub struct Identifier {
    pub label: String,
}

/// 式
pub struct Expression {
    pub tokens: Vec<Token>,
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    ILLEGAL,   // 未知のトークン
    EOF,       // ファイル終端
    IDENT,     // 識別子
    INT,       // 整数
    ASSIGN,    // イコール
    PLUS,      // 加算演算子
    COMMA,     // コンマ
    SEMICOLON, // セミコロン
    LPAREN,    // 左カッコ
    RPAREN,    // 右カッコ
    LBRACE,    // 左波括弧
    RBRACE,    // 右波括弧
    FUNCTION,  // 関数キーワード
    LET,       // 変数宣言キーワード
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, character: char) -> Token {
        Token {
            token_type,
            literal: character.to_string(),
        }
    }
}

#[cfg(test)]
mod tests_for_token {
    use crate::{Token, TokenType};

    #[test]
    fn test_new() {
        let token = Token::new(TokenType::COMMA, ',');
        assert_eq!(token.token_type, TokenType::COMMA);
        assert_eq!(token.literal, ",");
    }
}

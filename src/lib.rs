use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::lexer::Lexer;
use crate::token::TokenType;

pub mod lexer;
pub mod repl;
pub mod token;

#[wasm_bindgen]
pub struct ToriLang {}

#[wasm_bindgen]
impl ToriLang {
    pub fn tokenize(code: String) -> Result<JsValue, JsValue> {
        let mut lexer = Lexer::new(&code);
        let mut tokens: Vec<String> = vec![];
        loop {
            let token = lexer.get_next_token();
            tokens.push(token.literal);
            if token.token_type == TokenType::EOF {
                break;
            }
        }
        Ok(serde_wasm_bindgen::to_value(&tokens).unwrap())
    }
}

/* Token.js */

const Token = {
	ILLEGAL: "ILLEGAL",
	EOF: "EOF",

	// 識別子
	IDENT: "IDENT",

	// リテラル
	INTEGER: "INTEGER",
	FLOAT: "FLOAT",
	STRING: "STRING",

	// 演算子
	EQUAL: "EQUAL",
	PLUS: "PLUS",
	MINUS: "MINUS",

	// デリミタ
	COMMA: "COMMA", // ","
	SEMICOLON: "SEMICOLON", // ";"

	LPAREN: "LPAREN", // "("
	RPAREN: "RPAREN", // ")"
	LBRACE: "LBRACE", // "{"
	RBRACE: "RBRACE", // "}"

	// キーワード
	FUNCTION: "FUNCTION",
	LET: "LET",
}

module.exports = Token;
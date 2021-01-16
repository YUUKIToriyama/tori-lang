/* lexer.js */

const Token = require("./Token");
const TypeChecker = require("./TypeChecker");

const lexer = (code) => {
	let tokens = []; // トークン列(この配列に格納する)
	let chars = code.split(""); // 入力されたソースコードを1文字ずつに分割
	let readChars = "";
	while (chars.length > 0) {
		let token = {
			type: "",
			literal: ""
		}
		// ソースコードを1文字ずつ読んでいく
		let char = chars.shift();

		if ((char === " " || char === ";" || char === ",") && !/^["']/.test(readChars)) {
			// 区切り文字に達した時
			switch (readChars) {
				case "=": {
					token.type = Token.EQUAL;
					break;
				}
				case "+": {
					token.type = Token.PLUS;
					break;
				}
				case "-": {
					token.type = Token.MINUS;
					break;
				}
				case "let": {
					token.type = Token.LET;
					break;
				}
				case "func": {
					token.type = Token.FUNCTION;
					break;
				}
				// 予約語にない場合
				// TypeCheckerを用いて正規表現によるリテラル型判定を行なう
				default: {
					switch (TypeChecker.check(readChars)) {
						case "integer": {
							token.type = Token.INTEGER;
							break;
						}
						case "float": {
							token.type = Token.FLOAT;
							break;
						}
						// 文字列の場合、それは識別子である
						case "string": {
							token.type = Token.IDENT;
						}
					}
					break;
				}
			}
			token.literal = readChars;
			// 出来上がったトークンはトークン列tokensに入れる
			tokens.push(token);
			// デリミタもトークンとしてtokensに入れておく
			switch (char) {
				case ";": {
					tokens.push({ type: Token.SEMICOLON, literal: ";" });
					break;
				}
				case ",": {
					tokens.push({ type: Token.COMMA, literal: "," });
					break;
				}
			}
			// 読み取りが終わったreadCharsは初期化する
			readChars = "";
		} else if (/^["'].*["']$/.test(readChars + char)) {
			token.type = Token.STRING;
			token.literal = (readChars + char).replace(/^["'"]/, "").replace(/["']$/, "");
			tokens.push(token);
			readChars = "";
		} else {
			// 区切り文字に達していない時
			// あらたに読み込んだ文字をreadCharsの末尾に追加する
			readChars = readChars + char;
		}
	}
	return tokens;
}

module.exports = lexer;
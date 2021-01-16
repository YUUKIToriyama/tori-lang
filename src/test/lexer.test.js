const lexer = require("../lexer");

describe("lexer()", () => {
	test("lexer", () => {
		const code = `let x = 1;\nlet str = "Hello, world";`;
		const output = [
			{
				type: "LET",
				literal: "let"
			},
			{
				type: "IDENT",
				literal: "x"
			},
			{
				type: "EQUAL",
				literal: "="
			},
			{
				type: "INTEGER",
				literal: "1"
			},
			{
				type: "SEMICOLON",
				literal: ";"
			},
			{
				type: "LET",
				literal: "let"
			},
			{
				type: "IDENT",
				literal: "str"
			},
			{
				type: "EQUAL",
				literal: "="
			},
			{
				type: "STRING",
				literal: "Hello, world"
			},
			{
				type: "SEMICOLON",
				literal: ";"
			}
		];
		expect(lexer(code)).toEqual(output); //連想配列の同一性を確かめるときにはtoBe()ではなくtoEqual()を用いる
	});
});
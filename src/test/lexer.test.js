const lexer = require("../lexer");

describe("lexer()", () => {
	test("lexer", () => {
		const code = "Sum 10 100";
		const output = [
			{
				type: "command",
				value: "Sum"
			},
			{
				type: "integer",
				value: 10
			},
			{
				type: "integer",
				value: 100
			}
		];
		expect(lexer(code)).toEqual(output); //連想配列の同一性を確かめるときにはtoBe()ではなくtoEqual()を用いる
	});
});
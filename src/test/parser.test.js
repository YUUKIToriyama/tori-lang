const parser = require("../parser");

describe("parser()", () => {
	test("parser", () => {
		const tokens = [
			{
				type: "command",
				value: "Sub"
			},
			{
				type: "integer",
				value: 10
			},
			{
				type: "integer",
				value: 3
			}
		];
		const output = {
			type: "Program",
			body: [
				{
					type: "CallExpression",
					name: "Sub",
					arguments: [
						{
							type: "integer",
							value: 10
						},
						{
							type: "integer",
							value: 3
						}
					]
				}
			]
		};
		expect(parser(tokens)).toEqual(output);
	});
});
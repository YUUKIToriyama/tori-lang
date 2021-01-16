/* lexer.js */

const TypeChecker = require("./TypeChecker");

const lexer = (code) => {
	const keywords = code.split(/\s+/).filter(keyword => {
		return keyword.length > 0
	});
	const tokens = keywords.map((_keyword) => {
		switch (TypeChecker.check(_keyword)) {
			case "command":
				return {
					type: "command",
					value: _keyword
				}
				break;
			case "integer":
				return {
					type: "integer",
					value: parseInt(_keyword)
				}
				break;
			case "float":
				return {
					type: "float",
					value: parseFloat(_keyword)
				}
				break;
			case "string":
				return {
					type: "string",
					value: _keyword
				}
				break;
		}
	});
	return tokens;
}

module.exports = lexer;
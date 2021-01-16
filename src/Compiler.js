/* Compiler.js */

const lexer = require("./lexer");
const parser = require("./parser");
const generator = require("./generator");

class Compiler {
	name = "ToriLang Compiler";
	version = "0.0.1";
	credit = "(c)2021 YUUKIToriyama All Rights Reserved.";

	lexer = lexer;
	parser = parser;
	generator = generator;

	compile = (code) => {
		console.log(this.name + "\n" + this.credit);
		return this.generator(this.parser(this.lexer(code)));
	}
}

module.exports = Compiler;
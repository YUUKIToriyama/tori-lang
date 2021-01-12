/* main.js */

const reservedWords = [
	// 四則演算
	"Sum", "Sub", "Mul", "Div"
]

class TypeChecker {
	static isCommand(_str) {
		return reservedWords.includes(_str)
	}
	static isInteger(_str) {
		return /^[0-9]+$/.test(_str);
	}
	static isFloat(_str) {
		return /^[0-9]+\.[0-9]+$/.test(_str);
	}
	static isString(_str) {
		// 予約語でも、整数でも小数でもない場合文字列と判断する
		return !this.isCommand(_str) && !this.isInteger(_str) && !this.isFloat(_str);
	}

	static check(_str) {
		if (this.isCommand(_str)) {
			return "command";
		} else if (this.isInteger(_str)) {
			return "integer";
		} else if (this.isFloat(_str)) {
			return "float";
		} else {
			return "string";
		}
	}
}


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


const parser = (tokens) => {
	let AST = {
		type: "Program",
		body: []
	}

	while (tokens.length > 0) {
		let currentToken = tokens.shift();

		if (currentToken.type === "command") {
			let expression = {
				type: "CallExpression",
				name: null,
				arguments: []
			};
			// commandトークンにたどり着いたら構文解析を始める
			switch (currentToken.value) {
				case "Sum": {
					expression.name = "Sum";
					// トークン列を読み進め次を参照する
					let arguments = [tokens.shift(), tokens.shift()];
					if (arguments.every(_argument => ["integer", "float"].includes(_argument.type))) {
						// 2つがいずれも数であれば引数情報を変数expressionに格納
						expression.arguments = arguments;
						// expressionオブジェクトをASTのbodyに追加
						AST.body.push(expression);
					} else {
						throw new Error("Sum (number) (number)");
					}
					break;
				}

				case "Sub": {
					expression.name = "Sub";
					// トークン列を読み進め次を参照する
					let arguments = [tokens.shift(), tokens.shift()];
					if (arguments.every(_argument => ["integer", "float"].includes(_argument.type))) {
						// 2つがいずれも数であれば引数情報を変数expressionに格納
						expression.arguments = arguments;
						// expressionオブジェクトをASTのbodyに追加
						AST.body.push(expression);
					} else {
						throw new Error("Sub (number) (number)");
					}
					break;
				}

				case "Mul": {
					expression.name = "Mul";
					// トークン列を読み進め次を参照する
					let arguments = [tokens.shift(), tokens.shift()];
					if (arguments.every(_argument => ["integer", "float"].includes(_argument.type))) {
						// 2つがいずれも数であれば引数情報を変数expressionに格納
						expression.arguments = arguments;
						// expressionオブジェクトをASTのbodyに追加
						AST.body.push(expression);
					} else {
						throw new Error("Mul (number) (number)");
					}
					break;
				}

				case "Div": {
					expression.name = "Div";
					// トークン列を読み進め次を参照する
					let arguments = [tokens.shift(), tokens.shift()];
					if (arguments[1].value === 0) {
						// ゼロでは割れない 
						throw new Error("zero devide");
					} else {
						if (arguments.every(_argument => ["integer", "float"].includes(_argument.type))) {
							// 2つがいずれも数であれば引数情報を変数expressionに格納
							expression.arguments = arguments;
							// expressionオブジェクトをASTのbodyに追加
							AST.body.push(expression);
						} else {
							throw new Error("Div (number) (number)");
						}
					}
					break;
				}
			}
		}
	}
	return AST;
}


const generator = (ast) => {
	return ast.body.map(_expression => {
		switch (_expression.name) {
			case "Sum": {
				return _expression.arguments[0].value + _expression.arguments[1].value;
				break;
			}
			case "Sub": {
				return _expression.arguments[0].value - _expression.arguments[1].value;
				break;
			}
			case "Mul": {
				return _expression.arguments[0].value * _expression.arguments[1].value;
				break;
			}
			case "Div": {
				// 引数が整数か小数かで振る舞いを変える
				if (_expression.arguments.some(_elem => _elem.type === "float")) {
					return _expression.arguments[0].value / _expression.arguments[1].value;
				} else {
					return (_expression.arguments[0].value - (_expression.arguments[0].value % _expression.arguments[1].value)) / _expression.arguments[1].value;
				}
				break;
			}
		}
	})
}

class ToriLangCompiler {
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

const code = "Div 12 3";
const tcc = new ToriLangCompiler();
const result = tcc.compile(code);
console.log(result);
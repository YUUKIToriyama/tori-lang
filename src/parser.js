/* parser.js */

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

module.exports = parser;
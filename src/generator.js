/* generator.js */

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

module.exports = generator;
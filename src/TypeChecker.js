/* TypeChecker.js */


class TypeChecker {
	static isInteger(_str) {
		return /^[0-9]+$/.test(_str);
	}
	static isFloat(_str) {
		return /^[0-9]+\.[0-9]+$/.test(_str);
	}
	static isString(_str) {
		// 予約語でも、整数でも小数でもない場合文字列と判断する
		return !this.isInteger(_str) && !this.isFloat(_str);
	}

	static check(_str) {
		if (this.isInteger(_str)) {
			return "integer";
		} else if (this.isFloat(_str)) {
			return "float";
		} else {
			return "string";
		}
	}
}

module.exports = TypeChecker;
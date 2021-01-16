const { TestScheduler } = require('jest');
const TypeChecker = require("../TypeChecker");

describe("TypeChecker", () => {
	describe("isCommand()", () => {
		test("isCommand", () => {
			expect(TypeChecker.isCommand("Sum")).toBe(true);
		});
	});
	describe("isInteger()", () => {
		test("isInteger", () => {
			expect(TypeChecker.isInteger("1234500")).toBe(true);
		});
	});
	describe("isFloat()", () => {
		test("isFloat", () => {
			expect(TypeChecker.isFloat("3.1419")).toBe(true);
		});
	});
	describe("isString()", () => {
		test("isString", () => {
			expect(TypeChecker.isString("toriyama2021")).toBe(true);
		});
	});
});
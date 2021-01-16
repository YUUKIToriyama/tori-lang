/* main.js */

const Compiler = require("./Compiler");

const code = "Div 12 3";
const tcc = new Compiler();
const result = tcc.compile(code);
console.log(result);
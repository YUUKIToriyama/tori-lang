import init, { ToriLang } from "../pkg/compiler.js";

await init();

const inputArea = document.getElementById("input");
const outputArea = document.getElementById("output");
inputArea.addEventListener("keyup", event => {
	const tokens = ToriLang.tokenize(inputArea.value);
	outputArea.innerText = tokens.map(token => `"${token}"`).join(",");
});
/*
node dev.mjs | tee dev.log
cargo expand | tee cargo-expand.log
*/

import fs from "node:fs";

/**
 *
 * @param {string} wasmFile
 */
async function parse(wasmFile) {
	const data = await fs.promises.readFile(wasmFile);
	const module = await WebAssembly.compile(data);
	const imports = WebAssembly.Module.imports(module);
	const exports = WebAssembly.Module.exports(module);

	return {
		module,
		imports,
		exports,
	};
}

// wasm-bindgen modifies imports/exports to fit the target environment (e.g. bundler, web, ...)
for (const wasmFile of [
	"./target/wasm32-unknown-unknown/release/murmur3_wasm_bindgen.wasm",
	"./pkg/index_bg.wasm",
]) {
	console.log(wasmFile, await parse(wasmFile));
}

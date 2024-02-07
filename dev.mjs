// node dev.mjs

import fs from "node:fs";

const wasmFile = "./pkg/index_bg.wasm";
const data = await fs.promises.readFile(wasmFile);
const mod = await WebAssembly.compile(data);
const modImports = WebAssembly.Module.imports(mod);
const modExports = WebAssembly.Module.exports(mod);

console.log({
	mod,
	modImports,
	modExports,
});

// {
//   mod: Module [WebAssembly.Module] {},
//   modImports: [
//     {
//       module: 'wbg',
//       name: '__wbindgen_copy_to_typed_array',
//       kind: 'function'
//     },
//     { module: 'wbg', name: '__wbindgen_error_new', kind: 'function' },
//     {
//       module: 'wbg',
//       name: '__wbindgen_object_drop_ref',
//       kind: 'function'
//     }
//   ],
//   modExports: [
//     { name: 'memory', kind: 'memory' },
//     { name: 'murmur3_32', kind: 'function' },
//     { name: 'murmur3_x64_128', kind: 'function' },
//     { name: 'murmur3_x86_128', kind: 'function' },
//     { name: '__wbindgen_add_to_stack_pointer', kind: 'function' },
//     { name: '__wbindgen_malloc', kind: 'function' }
//   ]
// }

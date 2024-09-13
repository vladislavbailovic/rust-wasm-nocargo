let WASM;

async function init() {
	WASM = await WebAssembly.instantiateStreaming(await fetch("lib.wasm"), {})
	console.log(WASM)
}
init();

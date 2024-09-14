let WASM;

async function init() {
	WASM = await WebAssembly.instantiateStreaming(await fetch("lib.wasm"), {})
	console.log(WASM)

	WASM.instance.exports.init();

	const buffer_start = WASM.instance.exports.get_data();
	const buffer_len = WASM.instance.exports.get_len();
	const pixels = new Uint8ClampedArray(WASM.instance.exports.memory.buffer, buffer_start, buffer_len);
	const image = new ImageData(pixels, 16*5, 9*5);

	const canvas = document.getElementById("out");
	const ctx = canvas.getContext("2d");
	ctx.putImageData(image, 0, 0, 0, 0, 16*5, 9*5)

}
init();

let WASM;

async function init() {
	const mod = await WebAssembly.instantiateStreaming(await fetch("lib.wasm"), {})
	WASM = mod.instance.exports;
	console.log(WASM)

	WASM.init();

	const WIDTH = WASM.get_width();
	const HEIGHT = WASM.get_height();

	const buffer_start = WASM.get_data();
	const buffer_len = WASM.get_data_len();
	const pixels = new Uint8ClampedArray(WASM.memory.buffer, buffer_start, buffer_len);
	const image = new ImageData(pixels, WIDTH, HEIGHT);

	const canvas = document.getElementById("out");
	const ctx = canvas.getContext("2d");
	ctx.putImageData(image, 0, 0, 0, 0, WIDTH, HEIGHT)

}
init();

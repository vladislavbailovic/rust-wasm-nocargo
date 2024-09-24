let WASM;

async function init() {
	const mod = await WebAssembly.instantiateStreaming(await fetch("lib.wasm"), {
		env: {
			wasm_log: (ptr, len) => {
				const data = new Uint8ClampedArray(WASM.memory.buffer, ptr, len);
				const text = new TextDecoder().decode(data);
				console.log(`[WASM] ${text}`);
			},
		},
	});
	WASM = mod.instance.exports;

	WASM.init();

	const WIDTH = WASM.get_width();
	const HEIGHT = WASM.get_height();

	const buffer_start = WASM.get_data();
	const buffer_len = WASM.get_data_len();
	const pixels = new Uint8ClampedArray(WASM.memory.buffer, buffer_start, buffer_len);
	const image = new ImageData(pixels, WIDTH, HEIGHT);

	const canvas = document.getElementById("out");
	const ctx = canvas.getContext("2d");
	canvas.width = WIDTH;
	canvas.height = HEIGHT;
	ctx.putImageData(image, 0, 0, 0, 0, WIDTH, HEIGHT)

}
init();

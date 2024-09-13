serve: www/lib.wasm
	python -m http.server -d www 6660

www/lib.wasm: src/lib.rs Makefile
	# Thank you! https://surma.dev/things/rust-to-webassembly/
	rustc --crate-type=cdylib \
		--target=wasm32-unknown-unknown \
		-C opt-level='z' \
		-C lto=on \
		src/lib.rs -o lib.raw.wasm
	wasm-opt -O3 lib.raw.wasm -mvp -o www/lib.wasm
	rm lib.raw.wasm

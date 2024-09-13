SOURCE=src/lib.rs
BUILDDIR=build
WWWDIR=www

serve: $(WWWDIR)/lib.wasm test
	python -m http.server -d $(WWWDIR) 6660

test: src/lib.rs
	rustc --test $(SOURCE) -o $(BUILDDIR)/runall
	./$(BUILDDIR)/runall
	rm ./$(BUILDDIR)/runall

www/lib.wasm: $(BUILDDIR)/lib.raw.wasm
	wasm-opt -O3 $(BUILDDIR)/lib.raw.wasm -mvp -o $(WWWDIR)/lib.wasm
	rm $(BUILDDIR)/lib.raw.wasm

$(BUILDDIR)/lib.raw.wasm: src/lib.rs Makefile
	# Thank you! https://surma.dev/things/rust-to-webassembly/
	rustc --crate-type=cdylib \
		--target=wasm32-unknown-unknown \
		-C opt-level=s \
		-C lto=on \
		$(SOURCE) -o $(BUILDDIR)/lib.raw.wasm

SOURCE=src/lib.rs
BUILDDIR=build
WWWDIR=www

serve: build
	python -m http.server -d $(WWWDIR) 6660

build: $(WWWDIR)/lib.wasm test
	@echo "Built"

fmt:
	rustfmt src/*.rs

test: src/lib.rs fmt
	rustc --test $(SOURCE) -o $(BUILDDIR)/runall
	./$(BUILDDIR)/runall
	rm ./$(BUILDDIR)/runall

www/lib.wasm: $(BUILDDIR)/lib.raw.wasm
	# Optimized version segfaults? wtf o.0
	# wasm-opt -Os $(BUILDDIR)/lib.raw.wasm -mvp -o $(WWWDIR)/lib.wasm
	wasm-opt $(BUILDDIR)/lib.raw.wasm -mvp -o $(WWWDIR)/lib.wasm
	rm $(BUILDDIR)/lib.raw.wasm

$(BUILDDIR)/lib.raw.wasm: src/lib.rs Makefile src/map_data.rs
	# Thank you! https://surma.dev/things/rust-to-webassembly/
	rustc --crate-type=cdylib \
		--target=wasm32-unknown-unknown \
		-C opt-level=s \
		-C lto=on \
		$(SOURCE) -o $(BUILDDIR)/lib.raw.wasm

src/map_data.rs: bin/preprocessor.py data/map.osm
	./bin/preprocessor.py data/map.osm > src/map_data.rs

data/map.osm:
	curl 'https://api.openstreetmap.org/api/0.6/map?bbox=19.825164831789305,45.25099740300844,19.83664980744615,45.26115846317492' > data/map.osm

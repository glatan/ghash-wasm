NAME = ghash

.PHONY: build/wasm
build/wasm:
	@cargo build --target wasm32-unknown-unknown

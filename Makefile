.PHONY: dep
dep:
	wasm-tools component embed -w dep deps/dep.wit --dummy -o deps/dep.wasm
	wasm-tools component new deps/dep.wasm -o deps/dep.wasm
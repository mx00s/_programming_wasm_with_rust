# Depends on wasm-pack: see https://yew.rs/docs/en/getting-started/project-setup/using-wasm-pack

.PHONY: run clean

run:
	@wasm-pack build --target web --out-name wasm --out-dir ./static
	@cd static && python -m http.server 8000

clean:
	@cargo clean
	@git clean -dxf

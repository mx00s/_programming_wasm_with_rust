.PHONY: all run clean

all: target/wasm32-unknown-unknown/release/bindgenhello.wasm
	@wasm-bindgen target/wasm32-unknown-unknown/release/bindgenhello.wasm --out-dir .
	@npm install
	@npm run-script build

run: all
	@npm run-script serve

clean:
	@rm -f bindgenhello*
	@cargo clean

target/wasm32-unknown-unknown/release/bindgenhello.wasm:
	@cargo build --release --target wasm32-unknown-unknown

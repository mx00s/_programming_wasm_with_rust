.PHONY: all dump test clean

all: checkers.wasm

dump: checkers.wasm
	@wasm-objdump checkers.wasm -x -h -s -d

# Q6: Is there a lightweight way to run `func_test.js` headlessly (without the browser and `index.html`)?
# Q7: If Q6, is there a conventional testing tool for WASM modules?
test:
	@echo "After the server starts, open http://localhost:8000 in your browser and inspect the console output"
	@echo "Starting server..."
	@python3 -m http.server

clean:
	@rm checkers.wasm

checkers.wasm:
	@wat2wasm checkers.wat

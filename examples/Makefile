%.wasm: %.rs
	rustc $^ -o $@ --target=wasm32-unknown-unknown

all: echo.wasm add.wasm
default: build

all: test

test: build
	cargo test

build:
	cargo build --target wasm32-unknown-unknown --release 
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

saw-build:
	cargo saw-build --target wasm32-unknown-unknown --release

fmt:
	cargo fmt --all

clean:
	cargo clean

default: build

all: test

test: build
	cargo test

build:
	cargo build --target wasm32-unknown-unknown --release 
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

saw-build:
	cargo saw-build --target wasm32-unknown-unknown --release
	cp target/*/release/deps/*.linked-mir.json target/linked-mir.json

fmt:
	cargo fmt --all

clean:
	cargo clean

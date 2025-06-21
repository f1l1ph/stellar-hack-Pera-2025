default: build

all: test

test: build
	cargo test

build:
	stellar contract build
	@ls -l target/wasm32v1-none/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean

# this is not working
bindings:
	stellar contract build
	stellar contract bindings typescript \
		--wasm target/wasm32v1-none/release/options_contract.wasm \
		--output-dir bindings
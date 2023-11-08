
.PHONY: clean run

build:
	cargo build \
	--target wasm32-wasi \
	--release

clean:
	cargo clean
run:
	sudo envoy -c ./envoy.yaml \
	--concurrency 2 \
	--log-format '%v'


.PHONY: clean run

build:
	cargo build \
	--target wasm32-wasi \
	--release

clean:
	cargo clean
run: build
	sudo envoy -c ./envoy.yaml \
	--concurrency 2 \
	--log-level info  \
	--log-format '%v'

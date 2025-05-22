
.PHONY: clean run

build:
	cargo build \
	--target wasm32-wasip1 \
	--release

clean:
	cargo clean
run:
	envoy -c ./envoy.yaml \
	--concurrency 2 \
	--log-format '%v'

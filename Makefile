.PHONY: init
init:
	./scripts/init.sh

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check --release

.PHONY: test
test:
	.maintain/run-test-locally.sh

.PHONY: run
run:
	cargo run --release -- --dev --tmp
	#cargo run -- --dev --tmp

.PHONY: build-benchmark
build-benchmark:
	cargo build --release --features runtime-benchmarks

.PHONY: build
build:
	cargo build --release

.PHONY: clean
clean:
	cargo +nightly fmt
	cargo +nightly clippy

.PHONY: prepush
clean:
	cargo +nightly fmt
	cargo +nightly clippy
	.maintain/run-test-locally.sh

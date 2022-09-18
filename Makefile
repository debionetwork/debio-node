.PHONY: init
init:
	./scripts/init.sh

.PHONY: check
check:
	cargo +nightly fmt
	cargo +nightly clippy
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

.PHONY: benchmark
benchmark:
	.maintain/run-benchmark-locally-build-once.sh

.PHONY: build
build:
	cargo build --release

.PHONY: format
format:
	cargo +nightly fmt

.PHONY: prepush
prepush:
	cargo +nightly fmt
	cargo +nightly clippy
	.maintain/run-test-locally.sh

.PHONY: install-hooks
install-hooks:
	.maintain/install-hooks.sh

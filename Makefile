rs-files := $(wildcard src/*.rs)

.PHONY: build
build: target/release/ding

target/release/ding: $(rs-files) Cargo.toml Makefile
	cargo build --release

.PHONY: format
format:
	cargo fmt --verbose

.PHONY: test
test:
	cargo fmt -- --check
	cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic
	cargo test

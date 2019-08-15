.PHONY: build check test lint clippy format

build:
	cargo build --release

check:
	cargo check

clippy:
	rustup component add clippy; \
	cargo clippy --all --all-targets -- -Dwarnings -Drust-2018-idioms

format:
	rustup component add rustfmt; \
	cargo fmt --all -- --check

lint: format clippy

test:
	cargo test

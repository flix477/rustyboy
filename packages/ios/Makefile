.PHONY: build test lint test-lib lint-lib format-lib clippy-lib lint-app install-pods check

install-pods:
	./scripts/install-pods.sh

build: install-pods
	./scripts/build.sh

check:
	cargo check

test-lib:
	cargo test

test: test-lib

format-lib:
	rustup component add rustfmt; \
	cargo fmt --all -- --check

clippy-lib:
	rustup component add clippy; \
	cargo clippy --all --all-targets -- -Dwarnings -Drust-2018-idioms

lint-lib: format-lib clippy-lib

lint-app: install-pods
	./scripts/lint.sh

lint: lint-lib lint-app

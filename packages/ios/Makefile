.PHONY: build test lint test-lib lint-lib format-lib clippy-lib lint-app

build:
	./scripts/build.sh

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

lint-app:
	cd app; \
	./Pods/Swiftlint/swiftlint --strict

lint: lint-lib lint-app
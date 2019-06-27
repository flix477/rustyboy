.PHONY: build test lint build-lib link-lib build-app test-lib lint-lib format-lib clippy-lib lint-app

build-lib:
	./scripts/build-and-link.sh

build-app: build-lib
	cd frontend; npm run build

build: build-app

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
	cd frontend; \
 	npx eslint --ext .tsx --ext .ts src/ --max-warnings=0

lint: lint-lib lint-app

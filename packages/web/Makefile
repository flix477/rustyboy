.PHONY: build test lint build-lib link-lib build-app check check-app check-lib test-lib lint-lib format-lib clippy-lib lint-app install-dependencies

install-dependencies:
	cd frontend; \
	npm i

build-lib:
	./scripts/build-and-link.sh

build-app: install-dependencies build-lib
	cd frontend; \
	npm run build

build: build-app

check-lib:
	cargo check

check-app: install-dependencies build-lib
	cd frontend; \
	npx tsc

check: check-app

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

lint-app: install-dependencies
	cd frontend; \
 	npx eslint --ext .tsx --ext .ts src/ --max-warnings=0

lint: lint-lib lint-app

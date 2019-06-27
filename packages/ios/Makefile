.PHONY: build test lint build-lib build-app test-lib lint-lib format-lib clippy-lib lint-app

build-lib:
	cargo install cargo-lipo; \
	cargo lipo --release; \
	mkdir target/headers; \
	cbindgen src/lib.rs -l c > target/headers/rustyboy.h

build-app: build-lib
	cd app; \
	xcodebuild clean build -workspace rustyboy.xcworkspace -scheme rustyboy -destination "platform=iOS Simulator,name=iPhone Xs,OS=12.2" CODE_SIGN_IDENTITY="" CODE_SIGNING_REQUIRED=NO ONLY_ACTIVE_ARCH=NO -quiet

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
	cd app; \
	./Pods/Swiftlint/swiftlint --strict

lint: lint-lib lint-app

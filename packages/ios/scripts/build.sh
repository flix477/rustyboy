#!/bin/bash
if [[ "$OSTYPE" != "darwin"* ]]; then
    cargo check
    exit $?
fi

if ! cargo --list | grep -q lipo; then
    cargo install cargo-lipo
fi
cargo lipo --release
mkdir -p target/headers
cbindgen src/lib.rs -l c > target/headers/rustyboy.h

cd app
xcodebuild clean build -workspace rustyboy.xcworkspace -scheme rustyboy -destination "platform=iOS Simulator,name=iPhone Xs,OS=14.4" CODE_SIGN_IDENTITY="" CODE_SIGNING_REQUIRED=NO ONLY_ACTIVE_ARCH=NO -quiet

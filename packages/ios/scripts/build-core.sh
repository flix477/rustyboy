#!/bin/sh

REPO_ROOT=/Users/felixleveille/src/flix477/rustyboy

cd $REPO_ROOT/packages/core \
  && cargo build --release --target=aarch64-apple-ios \
  && cargo build --release --target=aarch64-apple-ios-sim \
  && cd ../uniffi-bindgen \
  && cargo run generate ../core/src/bindings.udl -l swift --out-dir ../core/target/ios-bindings \
  && cd ../core \
  && mv target/ios-bindings/rustyboy_coreFFI.modulemap target/ios-bindings/module.modulemap \
  && rm -rf target/RustyboyCore.xcframework \
  && xcodebuild -create-xcframework \
    -library target/aarch64-apple-ios/release/librustyboy_core.a \
    -headers target/ios-bindings \
    -library target/aarch64-apple-ios-sim/release/librustyboy_core.a \
    -headers target/ios-bindings \
    -output target/RustyboyCore.xcframework

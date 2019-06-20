#!/bin/sh

cargo lipo --release
mkdir target/headers
cbindgen src/lib.rs -l c > target/headers/rustyboy.h

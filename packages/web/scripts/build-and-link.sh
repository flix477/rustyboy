#!/bin/bash
if ! command -v wasm-pack; then
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 
fi
wasm-pack build

if [ ! -w pkg ]; then
    chown -R $USER:$USER pkg
fi

cd pkg
npm link
cd ../frontend
npm link rustyboy-web
cd ..

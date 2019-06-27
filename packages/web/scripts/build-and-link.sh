#!/bin/bash
if ! command -v wasm-pack; then
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 
fi
wasm-pack build

if [ ! -w pkg ]; then
    chown -R "$USER":"$USER" pkg
fi

cd pkg

if [ ! -w /usr/local/lib/node_modules ]; then
    sudo npm link
else
    npm link
fi

cd ../frontend
npm link rustyboy-web

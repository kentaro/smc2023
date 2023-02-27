#!/bin/sh

set -eu

cd liquid/

wasm-pack build

cp target/wasm32-unknown-unknown/release/liquid.wasm ../wasm
cd ..

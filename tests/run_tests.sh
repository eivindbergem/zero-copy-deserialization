#!/bin/bash

set -e

cargo run
cargo test
cross test --target arm-unknown-linux-gnueabihf
cross test --target armv5te-unknown-linux-gnueabi
cross test --target armv7-unknown-linux-gnueabihf
cross test --target powerpc-unknown-linux-gnu
cross test --target wasm32-unknown-emscripten

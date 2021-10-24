#!/bin/sh
rustup update nightly
rustup default nightly

cargo install cargo-binutils
rustup component add llvm-tools-preview
#!/bin/sh
cd "$(dirname "$0")"
set -e

cargo test

rustup component add clippy
cargo clippy

rustup component add rustfmt
cargo fmt
git diff --exit-code

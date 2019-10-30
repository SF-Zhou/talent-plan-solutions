#!/bin/sh
cd "$(dirname "$0")"
set -e

cargo run

rustup component add clippy
cargo clippy  # slow at first time

rustup component add rustfmt
cargo fmt
git diff --exit-code

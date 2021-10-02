#!/bin/sh

rustup toolchain install nightly --allow-downgrade

rustup component add clippy
rustup component add rustfmt

cargo install cargo-audit
cargo install cargo-edit
cargo install cargo-expand
cargo install cargo-tarpaulin
cargo install cargo-watch

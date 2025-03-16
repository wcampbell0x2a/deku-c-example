#/bin/bash
set -ex

cross build --release --target x86_64-unknown-linux-musl
cbindgen --crate deku-c-example > header.h
scuba --image ghcr.io/cross-rs/x86_64-unknown-linux-musl:edge x86_64-linux-musl-gcc -static -o test test.c target/x86_64-unknown-linux-musl/release/libdeku_c_example.a

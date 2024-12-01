#!/bin/bash

set -euxo pipefail

TARGETS=("thumbv6m-none-eabi" "thumbv7em-none-eabi" "thumbv7em-none-eabihf")
FEATURES=("stm32")

for TARGET in "${TARGETS[@]}"; do
    rustup target add "$TARGET"
    cargo build --target "$TARGET"
    cargo clippy -- --deny warnings

    for FEATURE in "${FEATURES[@]}"; do
        cargo build --features "$FEATURE" --target "$TARGET"
        cargo test --features "$FEATURE"
        cargo clippy -- --deny warnings
    done
done

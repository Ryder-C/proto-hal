#!/bin/bash

set -euxo pipefail

TARGETS=("thumbv6m-none-eabi" "thumbv7em-none-eabi" "thumbv7em-none-eabihf")
FEATURES=("stm32")

for TARGET in "${TARGETS[@]}"; do
    rustup target add "$TARGET"
    cargo build --target "$TARGET"
    cargo clippy -- --deny warnings

    # test proto-hal features
    for FEATURE in "${FEATURES[@]}"; do
        cargo build -p proto-hal --features "$FEATURE" --target "$TARGET"
        cargo test -p proto-hal --features "$FEATURE"
        cargo clippy -p proto-hal -- --deny warnings
    done
done

# temporary
cd stm32/g4
cargo build --all-features
cargo build --tests

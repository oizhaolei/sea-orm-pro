#!/bin/bash
set -e
if [ -d ./build_tools ]; then
    targets=(
        "Cargo.toml"
        "migration/Cargo.toml"
        "sea-orm-pro/Cargo.toml"
    )

    for target in "${targets[@]}"; do
        echo "cargo +nightly fmt --manifest-path ${target} --all"
        cargo +nightly fmt --manifest-path "${target}" --all
    done
else
    echo "Please execute this script from the repository root."
fi

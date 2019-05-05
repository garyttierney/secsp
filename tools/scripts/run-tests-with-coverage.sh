#!/bin/sh

SCRIPT=$(readlink -f "$0")
SCRIPT_PATH=$(dirname "$SCRIPT")
ROOT_PATH="$SCRIPT_PATH/../.."
CONVERTER_PATH="$ROOT_PATH/tools/coverage-converter/lcov_cobertura.py"

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"

set -x
cargo +nightly clean
cargo +nightly build --verbose
cargo +nightly test --verbose

zip -0 ccov.zip `find . \( -name "secsp*.gc*" \) -print`;
grcov ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing --ignore-dir "/*" -o lcov.info;
python "$CONVERTER_PATH" lcov.info

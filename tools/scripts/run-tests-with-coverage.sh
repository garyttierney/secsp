#!/bin/bash

SCRIPT_PATH="$(cd "$(dirname "$0")" && pwd)"
ROOT_PATH="$SCRIPT_PATH/../.."
CONVERTER_PATH="$ROOT_PATH/tools/coverage-converter/lcov_cobertura.py"

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"

set -x
cargo +nightly clean
cargo +nightly build --verbose
cargo +nightly test --verbose --format json > test_results.json


zip -0 ccov.zip `find . \( -name "secsp*.gc*" \) -print`;
grcov ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing --ignore-dir "/*" -o lcov.info;
bash <(curl -s https://codecov.io/bash) -t "${CODECOV_TOKEN}";

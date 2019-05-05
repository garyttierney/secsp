#!/bin/sh

SCRIPT=$(readlink -f "$0")
SCRIPT_PATH=$(dirname "$SCRIPT")
FUZZ_PKG_PATH="$SCRIPT_PATH/.."
ROOT_PATH="$FUZZ_PKG_PATH/../.."
EXAMPLES_PATH="$ROOT_PATH/documentation/examples"
OUTPUT_PATH="$ROOT_PATH/fuzzer-output"

if [ -z "$FUZZER_DURATION" ]; then
    FUZZER_DURATION="5m"
fi

fuzz() {
    mkdir -p "$OUTPUT_PATH"
    cargo +nightly afl build --manifest-path="$FUZZ_PKG_PATH/Cargo.toml"
    cargo +nightly afl fuzz -i "$EXAMPLES_PATH" -o "$OUTPUT_PATH" \
        "$FUZZ_PKG_PATH/target/debug/secsp-fuzzer" > /dev/null &

    FUZZER_PID=$!;

    if ! kill -0 $FUZZER_PID > /dev/null 2>&1; then
        echo "Fuzzer didn't start" >&2
    else
        sleep "$FUZZER_DURATION"
        kill -INT $FUZZER_PID
    fi
}

fuzz_report() {
    publish_report() {
        SUCCESS_PATTERN="$1"
        GLOB="$2"

        if ! grep "$SUCCESS_PATTERN" "$OUTPUT_PATH/fuzzer_stats" ; then
            REPORT_URL=$(tail -n +1 $GLOB | curl -F "sprunge=<-" http://sprunge.us)

            echo "Pattern not found: $SUCCESS_PATTERN" >&2
            echo "Report for $GLOB published to $REPORT_URL" >&2
        fi
    }

    publish_report "unique_crashes *: 0"  "$OUTPUT_PATH/crashes/*"
    publish_report "unique_hangs *: 0"  "$OUTPUT_PATH/hangs/*"

    unset publish_report
}

set -x

fuzz
fuzz_report

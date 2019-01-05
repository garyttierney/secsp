#!/bin/sh

WS_DIR="$PWD"

finish() {
    cd "$WS_DIR"
}

trap finish EXIT

cd "$WS_DIR/packages/secsp_fuzzer"

cargo afl build

echo "Running AFL"

timeout --signal=INT 5m cargo afl fuzz -i ./../../documentation/examples -o out target/debug/secsp-fuzzer > /dev/null || true
cat out/fuzzer_stats

if ! grep "unique_crashes *: 0" out/fuzzer_stats ; then
    echo "Running AFL produced crashes, build failed"
    CRASHES_URL=$(tail -n +1 out/crashes/* | curl -F "sprunge=<-" http://sprunge.us)
    HANGS_URL=$(tail -n +1 out/hangs/* | curl -F "sprunge=<-" http://sprunge.us)
    echo "crashes: $CRASHES_URL hangs: $HANGS_URL"
    exit 1
else
    echo "Running AFL produced no crashes!"
fi

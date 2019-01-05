#!/bin/sh

set -v

ORIGINAL_BRANCH=$(git rev-parse --abbrev-ref HEAD)
WS_DIR="$PWD"

if [ -z "$BASELINE_BRANCH" ]; then
    BASELINE_BRANCH="master"
fi

finish() {
    CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
    if [ ! "$CURRENT_BRANCH" = "$ORIGINAL_BRANCH" ]; then
        (cd "$WS_DIR" && git checkout "$ORIGINAL_BRANCH")
    fi
}

trap finish EXIT

if [ ! -f "$PWD/Cargo.toml" ]; then
    echo "ERROR: Cannot find Cargo manifest."
    exit 1
fi

if ! git fetch ; then
    echo "ERROR: Unable to fetch remote refs"
    exit 1
fi

if ! git checkout "$BASELINE_BRANCH" ; then
    echo "ERROR: Unable to checkout baseline branch"
    exit 1
fi

if ! cargo bench -- --save-baseline baseline ; then
    echo "ERROR: Failed to create change benchmark"
    exit 1
fi

if ! git checkout "$ORIGINAL_BRANCH" ; then
    echo "ERROR: Unable to checkout original ref"
    exit 1
fi

BENCHMARK_LOG_FILE=$(mktemp)
if ! cargo bench --quiet -- -b baseline > "$BENCHMARK_LOG_FILE" ; then
    echo "ERROR: Unable to create baseline benchmark"
    exit 78
fi

echo "RESULT:"
cat "$BENCHMARK_LOG_FILE"

if grep --quiet -iE "no change in performance|change within noise threshold" "$BENCHMARK_LOG_FILE" ; then
    echo "OK: No performance regressions found"
    exit 0
else
    echo "WARNING: Found performance regressions."
    exit 78
fi

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

if ! cargo bench -- --save-baseline change ; then
    echo "ERROR: Failed to create change benchmark"
    exit 1
fi

if ! git checkout "$BASELINE_BRANCH" ; then
    echo "ERROR: Unable to checkout baseline branch"
    exit 1
fi

if ! cargo bench -- --save-baseline baseline ; then
    echo "ERROR: Unable to create baseline benchmark"
    exit 78
fi

if ! cargo install --force critcmp ; then
    echo "ERROR: Failed to install critcmp"
    exit 1
fi

BENCHMARK_LOG_FILE=$(mktemp)

if ! critcmp baseline change -t 5 > "$BENCHMARK_LOG_FILE" ; then
    echo "ERROR: Failed to create benchmark diff"
    exit 1
fi

LOG=$(cat "$BENCHMARK_LOG_FILE")

echo "RESULT:"
cat "$BENCHMARK_LOG_FILE"

if [ -z "$LOG" ]; then
    echo "OK: No performance regressions found"
    exit 0
else
    echo "WARNING: Found performance regressions."

    if [ -n "$GITHUB_TOKEN" ]; then
        COMMENT_PAYLOAD=$(echo "{}" | jq --arg body "$LOG" '.body = $body')
        COMMENTS_URL=$(jq -r ".pull_request.comments_url" < /github/workflow/event.json)

        [ -n "$COMMENTS_URL" ] && curl -s -S -H "Authorization: token $GITHUB_TOKEN" --header "Content-Type: application/json" --data "$COMMENT_PAYLOAD" "$COMMENTS_URL" > /dev/null
    fi
fi

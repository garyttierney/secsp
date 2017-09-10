#!/bin/sh

pushd `dirname $0` > /dev/null
SCRIPTPATH=`pwd`
popd > /dev/null

if [ -z "$CSPC" ]; then 
    CSPC="$SCRIPTPATH/../target/debug/cspc"
fi

if [ -z "$SEXP_CMP" ]; then
    SEXP_CMP="$SCRIPTPATH/../target/debug/sexp_cmp"
fi

TEST_OUT_DIR="$SCRIPTPATH/test_out"
rm -Rf "$TEST_OUT_DIR"
mkdir -p "$TEST_OUT_DIR"

for CSP_FILE in $(find "$SCRIPTPATH" -name '*.csp'); do
    DIRNAME=$(dirname "$CSP_FILE")
    FILE_PREFIX=$(basename "$CSP_FILE" ".csp")
    CIL_FILE="$DIRNAME/$FILE_PREFIX.cil"
    CIL_OUTPUT_FILE="$TEST_OUT_DIR/$FILE_PREFIX.cil"
    sh -c "$CSPC -f '$CSP_FILE' > '$CIL_OUTPUT_FILE'"

    $SEXP_CMP "$CIL_FILE" "$CIL_OUTPUT_FILE"

    if [ $? -eq 1 ]; then
        echo "$FILE_PREFIX FAILED!"
    else
        echo "$FILE_PREFIX PASSED"
    fi
done
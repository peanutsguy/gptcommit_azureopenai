#!/bin/sh
set -eu

(
    rm -rf test_dir_foo4
    mkdir test_dir_foo4
    cd test_dir_foo4
    git init

    export TEMPFILE=$(mktemp)
    echo "foo" > $TEMPFILE

    GPTCOMMIT__OPENAI__MODEL="text-ada-001" \
    gptcommit_azureopenai prepare-commit-msg \
      --git-diff-content ../tests/data/example_1.diff \
      --commit-msg-file $TEMPFILE \
      --commit-source ""

    cat $TEMPFILE
)
rm -rf test_dir_foo4

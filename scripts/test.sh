#!/bin/bash

tests_dir=$(cd $(dirname $0); cd ../tests; pwd)
tests=(
    "envl-js-test"
    "envl-ts-test"
)

cargo test --verbose

for test_dir in ${tests[@]}; do
    cd $tests_dir
    cd $test_dir
    ./test.sh
    npm run check --if-present
done

#!/bin/bash

tests_dir=$(cd $(dirname $0); cd ../tests; pwd)
tests=(
    "envl-js-test"
)

cargo test --verbose

for test_dir in ${tests[@]}; do
    cd $tests_dir
    cd $test_dir
    ./test.sh
done

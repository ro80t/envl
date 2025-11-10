#!/bin/bash

script_dir=$(cd $(dirname $0); pwd)
scripts=(
    "build-release.sh"
    "test.sh"
    "lint.sh"
    "fmt.sh"
)

for script in ${scripts[@]}; do
    "${script_dir}/${script}"
    echo "Completed ${script}"
done

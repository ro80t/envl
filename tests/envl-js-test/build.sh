#!/bin/bash

envl=$(cd $(dirname $0); cd ../../target/release; pwd)/envl

$envl build ./src/envl.js

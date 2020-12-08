#!/bin/bash

cargo new $1 --name="aoc-2020-$1"

echo "aoc-2020-common = { path = \"../common/\" }" >> ./$1/Cargo.toml


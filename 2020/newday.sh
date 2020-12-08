#!/bin/bash

cargo new $1 --name="aoc-2020-$1"

echo "aoc-2020-common = { path = \"../common/\" }" >> ./$1/Cargo.toml

mkdir $1/input

touch $1/input/1.txt

echo 'use aoc_2020_common::common::load_file;

fn main() {
    let input = load_file("./input/1.txt");
    println!("{}", input);
}' > ./$1/src/main.rs

clion $1

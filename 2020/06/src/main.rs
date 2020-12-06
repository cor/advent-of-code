use std::fs::File;
use std::io::Read;
use itertools::Itertools;


fn load_file(path: &str) -> String {
    let mut input = String::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_string(&mut input).expect("Unable to read string");

    input
}

fn main() {
    let input = load_file("./input/1.txt");

    let answer1: usize = input
        .split("\n\n")
        .map(|x| x.to_string().replace("\n", ""))
        .map(|g| g.chars().into_iter().unique().count())
        .sum();

    println!("{:?}", answer1);
}

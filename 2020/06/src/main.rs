use std::fs::File;
use std::io::Read;
use itertools::Itertools;
use std::collections::HashMap;


fn load_file(path: &str) -> String {
    let mut input = String::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_string(&mut input).expect("Unable to read string");

    input
}

fn main() {
    let input = load_file("./input/1.txt");
    let groups = input.split("\n\n");

    let answer1: usize = groups.clone()
        .map(|s| s.to_string().replace("\n", ""))
        .map(|g| g.chars().into_iter().unique().count())
        .sum();

    let answer2: usize = groups
        .map(|group| (group.clone().lines().count(), group))
        .map(|(answer_count, answers)| {
            let mut char_counts: HashMap<char, usize> = HashMap::new();
            for c in answers.chars() {
                let count = char_counts.entry(c).or_insert(0);
                *count += 1;
            }

            char_counts.values()
                .filter(|&&count| count == answer_count)
                .count()
        })
        .sum();

    println!("{:?}", answer1);
    println!("{:?}", answer2);
}
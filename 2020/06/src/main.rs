use itertools::Itertools;
use std::collections::HashMap;
use aoc_2020_common::common::load_file;

fn main() {
    let input = load_file("./input/1.txt");
    let groups = input.split("\n\n");

    let answer1: usize = groups.clone()
        .map(|s| s.to_string().replace("\n", ""))
        .map(|g| g.chars().into_iter().unique().count())
        .sum();

    let answer2: usize = groups
        .map(|group| (group.lines().count(), group))
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
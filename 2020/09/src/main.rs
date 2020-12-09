use aoc_2020_common::common::load_file;

fn main() {
    let input = load_file("./input/1.txt");
    let numbers:Vec<usize> = input
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{:?}", numbers);
}

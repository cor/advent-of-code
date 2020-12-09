use aoc_2020_common::common::load_file;

fn numbers_contain_summing_pair(numbers: &[usize], target: &usize) -> bool {
    for (i, n) in numbers.iter().enumerate() {
        for (j, m) in numbers.iter().enumerate() {
            if i != j && n + m == *target {
                return true
            }
        }
    }
    false
}

fn main() {
    let input = load_file("./input/1.txt");
    let numbers:Vec<usize> = input
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    let range_size = 25;
    
    for (i, n) in numbers.iter().enumerate() {
        if i > range_size {
            let preceeding = &numbers[(i-range_size)..i];
            if !numbers_contain_summing_pair(&preceeding, n) {
                println!("{}", n);
            }
        }
    }
}

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

    // Part 1 answer
    let range_size = 25;
    let mut part_2_target: usize = 0;
    for (i, n) in numbers.iter().enumerate() {
        if i > range_size {
            let preceding = &numbers[(i-range_size)..i];
            if !numbers_contain_summing_pair(&preceding, n) {
                part_2_target = *n;
                println!("{}", n);
            }
        }
    }

    assert_ne!(part_2_target, 0);

    // Part 2 answer
    for (i, n) in numbers.iter().enumerate() {
        for (j, m) in numbers[i..].iter().enumerate() {
            let range = &numbers[i..(i+j)];
            if range.iter().sum::<usize>() == part_2_target {
                let smallest = range.iter().min().unwrap();
                let largest = range.iter().max().unwrap();
                println!("smallest: {}, largest: {}, sum: {}", smallest, largest, (smallest+largest));
            }
        }
    }
}

use aoc_2024_common::challenge_input;

fn main() {
    let input = challenge_input();
    println!("{input}");
    
    let parsed:  Vec<Vec<usize>> = input.lines().map(|l| l.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect()).collect();
    dbg!(parsed);

    // let parsed: Vec<Vec<usize> = input.lines()
    // .map(|l| l.split_whitespace().map(|n| n.parse()).collect()).collect();
}

use aoc_2022_common::challenge_input;

fn main() {
    let input: String = challenge_input();
    let lines = input.lines().collect::<Vec<_>>();
    let mut split_iter = lines.split(|l| l.is_empty());
    let containers = split_iter.next().expect("Missing containers").to_vec();
    let instructions = split_iter.next().expect("Missing instructions").to_vec();

    dbg!(&containers);

    let mut containers: Vec<Vec<char>> = transpose(
        containers[..containers.len() - 1]
            .iter()
            .map(|l| {
                l.chars()
                    .collect::<Vec<_>>()
                    .chunks(4)
                    .map(|c| c[1])
                    .collect()
            })
            .collect(),
    )
    .iter()
    .map(|container| {
        container
            .iter()
            .rev()
            .filter(|&c| *c != ' ')
            .copied()
            .collect()
    })
    .collect();

    containers[1].pop();

    dbg!(containers);
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

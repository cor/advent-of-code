use aoc_2022_common::challenge_input;

#[derive(Debug, Clone)]
struct Number {
    value: i64,
    index: usize,
}

fn mix(input_numbers: &[Number], mixing_numbers: &mut Vec<Number>) {
    let len_when_moving = (input_numbers.len() - 1) as i64;

    for num in input_numbers {
        let current_index = mixing_numbers
            .iter()
            .position(|n| n.index == num.index)
            .unwrap() as i64;

        let new_index =
            ((current_index + num.value) % len_when_moving + len_when_moving) % len_when_moving;

        let removed = mixing_numbers.remove(current_index as usize);
        mixing_numbers.insert(new_index as usize, removed);
    }
}

fn grove_coordinates_sum(mixed_numbers: &[Number]) -> i64 {
    let len = &mixed_numbers.len();
    let zero_index = mixed_numbers.iter().position(|n| n.value == 0).unwrap();

    mixed_numbers[(zero_index + 1000) % len].value
        + mixed_numbers[(zero_index + 2000) % len].value
        + mixed_numbers[(zero_index + 3000) % len].value
}

fn main() {
    let input = challenge_input();
    let input_numbers: Vec<Number> = input
        .lines()
        .enumerate()
        .map(|(index, val)| Number {
            value: val.parse().unwrap(),
            index,
        })
        .collect();

    let mut mixing_numbers = input_numbers.clone();
    mix(&input_numbers, &mut mixing_numbers);
    let part_1 = grove_coordinates_sum(&mixing_numbers);
    println!("{part_1}");

    let decryption_key = 811589153;
    let input_numbers_2 = input_numbers
        .iter()
        .map(|n| Number {
            index: n.index,
            value: n.value * decryption_key,
        })
        .collect::<Vec<_>>();

    let mut mixing_numbers_2 = input_numbers_2.clone();

    for _ in 0..10 {
        mix(&input_numbers_2, &mut mixing_numbers_2);
    }
    let part_2 = grove_coordinates_sum(&mixing_numbers_2);
    println!("{part_2}");
}

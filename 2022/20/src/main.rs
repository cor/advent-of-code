use aoc_2022_common::challenge_input;

#[derive(Debug, Clone)]
struct Number {
    value: i64,
    index: usize,
}

fn mix(input_numbers: &[Number], moving_numbers: &mut Vec<Number>) {
    let len_when_moving = (input_numbers.len() - 1) as i64;

    for num in input_numbers {
        let current_index = moving_numbers
            .iter()
            .position(|n| n.index == num.index)
            .unwrap() as i64;

        let new_index =
            ((current_index + num.value) % len_when_moving + len_when_moving) % len_when_moving;

        let removed = moving_numbers.remove(current_index as usize);
        moving_numbers.insert(new_index as usize, removed);
    }
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

    let mut moving_numbers = input_numbers.clone();

    mix(&input_numbers, &mut moving_numbers);

    let len = &input_numbers.len();
    let zero_index = moving_numbers.iter().position(|n| n.value == 0).unwrap();
    let sum = moving_numbers[(zero_index + 1000) % len].value
        + moving_numbers[(zero_index + 2000) % len].value
        + moving_numbers[(zero_index + 3000) % len].value;
    println!("{sum}");
}

use aoc_2023_common::challenge_input;

// fn print_schematic(schematic: Vec<Vec<char>>) {
//     for line in schematic {
//         for char in line {
//             print!("{char}");
//         }
//         println!();
//     }
// }

#[derive(Debug)]
struct PartNumber {
    number: usize,
    x: (usize, usize),
    y: usize,
}

impl PartNumber {
    fn is_valid(&self, schematic: &[Vec<char>], width: i32, height: i32) -> bool {
        for x in self.x.0..=self.x.1 {
            for (dx, dy) in NEIGHBORS {
                let new_x = x as i32 + dx;
                let new_y = self.y as i32 + dy;
                if new_x >= 0 && new_x < width && new_y >= 0 && new_y < height {
                    let z = schematic[new_y as usize][new_x as usize];
                    match z {
                        '0'..='9' | '.' => {}
                        _ => return true,
                    }
                }
            }
        }
        false
    }
}

fn find_part_numbers(schematic: &[Vec<char>]) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = Vec::new();

    for (y, line) in schematic.iter().enumerate() {
        let mut char_iter = line.iter().enumerate();
        while let Some((x0, &char)) = char_iter.next() {
            if char.is_ascii_digit() {
                let mut number_digits = vec![char];
                let mut x1 = x0;
                while let Some((char_index_inner, &char_inner @ '0'..='9')) = char_iter.next() {
                    number_digits.push(char_inner);
                    x1 = char_index_inner;
                }
                let number = number_digits.iter().collect::<String>().parse().unwrap();
                part_numbers.push(PartNumber {
                    number,
                    x: (x0, x1),
                    y,
                });
            }
        }
        println!();
    }
    part_numbers
}

const NEIGHBORS: [(i32, i32); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn main() {
    let input = challenge_input();
    let schematic: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let part_numbers = find_part_numbers(&schematic);
    let height = schematic.len() as i32;
    let width = schematic.first().unwrap().len() as i32;

    let part_1: usize = part_numbers
        .iter()
        .filter(|part_number| part_number.is_valid(&schematic, width, height))
        .map(|part| part.number)
        .sum();
    println!("{part_1}");
}

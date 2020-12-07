use std::str::FromStr;
use aoc_2020_common::common::load_file;

#[derive(Debug)]
struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    fn id(&self) -> usize {
        self.row * 8 + self.column
    }
}

// both bounds are inclusive.
#[derive(Debug)]
struct Range {
    lower: usize,
    upper: usize,
}

impl Range {
    fn take_lower_half(&mut self) {
        let half = (self.upper - self.lower) / 2;
        self.upper = self.lower + half;
    }

    fn take_upper_half(&mut self) {
        let half = (self.upper - self.lower) / 2;
        self.lower = self.lower + half + 1;
    }
}

impl FromStr for Seat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Determine Row
        let mut row_range = Range { lower: 0, upper: 127};
        for c in (&s[..(s.len()-3)]).chars() {
            match c {
                'F' => row_range.take_lower_half(),
                'B' => row_range.take_upper_half(),
                _   => panic!("Invalid row char in input"),
            }
        }
        let row = row_range.lower;

        // Determine Column
        let mut column_range = Range { lower: 0, upper: 7};
        for c in (&s[(s.len()-3)..]).chars() {
            match c {
                'L' => column_range.take_lower_half(),
                'R' => column_range.take_upper_half(),
                _   => panic!("Invalid row char in input"),
            }
        }
        let column = column_range.lower;

        Ok(Seat { row, column })
    }
}

fn main() {
    let input = load_file("./input/1.txt");

    // Get Seats from input
    let seats: Vec<Seat> = input
        .lines()
        .map(|s| Seat::from_str(s))
        .filter_map(Result::ok)
        .collect();

    // Map to IDs
    let mut seat_ids: Vec<usize> = seats
        .iter()
        .map(|s| s.id())
        .collect();
    seat_ids.sort();

    // Answer 1
    let answer_1 = seat_ids.iter().max();
    if let Some(ans) = answer_1 {
        println!("{:?}", ans);
    }

    // Answer 2
    for i in 0..(seat_ids.len() - 2) {
        if seat_ids[i+1] == seat_ids[i] + 2 {
            println!("{:?}", seat_ids[i] + 1);
        }
    }

}

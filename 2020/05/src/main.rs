use std::fs::File;
use std::io::Read;
use std::str::FromStr;

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


fn load_file(path: &str) -> String {
    let mut input = String::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_string(&mut input).expect("Unable to read string");

    input
}

fn main() {
    let input = load_file("./input/1.txt");

    let seats: Vec<Seat> = input
        .lines()
        .map(|s| Seat::from_str(s))
        .filter_map(Result::ok)
        .collect();

    let mut seat_ids: Vec<usize> = seats
        .iter()
        .map(|s| s.id())
        .collect();

    seat_ids.sort();

    println!("{:?}", seat_ids);

    let answer_1 = seat_ids.iter().max();
    println!("{:?}", answer_1);

    for (i, id) in seat_ids.iter().enumerate() {
        if i < (seat_ids.len() - 2) && seat_ids[i+1] == id + 2 {
            println!("{:?}", id + 1);
        }
    }

}

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
        let mut range = Range { lower: 0, upper: 127};

        Ok(Seat { row: 4, column: 4 }) // TODO: Calculate based on s
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

    let a_seat = Seat {
        row: 70,
        column: 7,
    };

    let mut a_range = Range {
        lower: 0,
        upper: 63,
    };

    println!("Before lower: {:?}", a_range);

    a_range.take_upper_half();

    println!("After lower: {:?}", a_range);

    println!("{:?}", a_seat);
    println!("{:?}", seats);
    println!("{:?}", a_seat.id());
}

use aoc_2020_common::common::load_file;
use std::ops::Index;

#[derive(Debug)]
enum Field {
    Floor,
    Seat,
    Occupied,
}

impl Field {
    fn to_char(&self) -> char {
        match self {
            Field::Floor => '.',
            Field::Seat => 'L',
            Field::Occupied => '#',
        }
    }
}

#[derive(Debug)]
struct Area {
    width: usize,
    height: usize,
    fields: Vec<Field>
}

impl Area {
    fn from(s: &str) ->  Area {
        // let lines = s.lines();

        let fields: Vec<Field> = s.lines().flat_map(|s| s.chars().map(|c| {
            match c {
                '.' => Field::Floor,
                'L' => Field::Seat,
                '#' => Field::Occupied,
                _ => panic!("Invalid char in input"),
            }
        })
        ).collect();

        Area {
            width: s.lines().next().unwrap().len(),
            height: s.lines().count(),
            fields
        }
    }
}

impl Index<(usize, usize)> for Area {
    type Output = Field;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 >= self.width || index.1 >= self.height {
            panic!("Accessing out of range");
        }

        return &self.fields[index.0 + index.1 * self.width];
    }
}

impl ToString for Area {
    fn to_string(&self) -> String {
        let mut s = String::new();

        for (c, f) in self.fields.iter().enumerate() {
            if c % self.width == 0 {
                s.push('\n');
            }
            s.push(f.to_char());
        }

        s
    }
}


fn main() {
    let input = load_file("./input/1.txt");
    let area = Area::from(&input);
    println!("{}", area.to_string());
}

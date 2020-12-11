use aoc_2020_common::common::load_file;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Clone)]
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

    fn next(&self) -> Area {
        let mut a = self.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let point = (x,y);
                if self[point] == Field::Floor { continue }
                a[point] = match self.adjacent_occupied_count(&point) {
                    0 => Field::Occupied,
                    x if x >= 4 => Field::Seat,
                    _ => continue,
                }
            }
        }
        a
    }

    fn in_bounds(&self, point: &(isize, isize)) -> bool {
        let (x, y) = point;


        *x > 0 && *x < self.width as isize && *y > 0 && *y < self.height as isize
    }

    fn adjacent_occupied_count(&self, point: &(usize, usize)) -> usize {
        let (x, y) = (point.0 as isize, point.1 as isize);
        [
            (x-1, y-1), (x-1, y), (x-1, y+1),
            (x, y-1), (x, y+1),
            (x+1, y-1), (x+1, y), (x+1, y+1),
        ].iter()
            .filter(|&p| self.in_bounds(p))
            .map(|&(x, y)| (x as usize, y as usize))
            .filter(|&p| self[p] == Field::Occupied)
            .count()
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

impl IndexMut<(usize, usize)> for Area {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.0 >= self.width || index.1 >= self.height {
            panic!("Accessing out of range");
        }

        return &mut self.fields[index.0 + index.1 * self.width];
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
    let input = load_file("./input/example.txt");
    let area = Area::from(&input);
    println!("{}", area.to_string());

    let area2 = area.next();
    println!("{}", area2.to_string());

}

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

#[derive(Debug, Clone, Eq, PartialEq)]
struct Area {
    width: usize,
    height: usize,
    fields: Vec<Field>
}

impl Area {

    /// Parses a `string` into an `Area`
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

    /// Runs the part 1 simulation
    fn simulation_1(&self) -> usize {
        let mut area = self.clone();
        loop {
            let old_area = area.clone();
            area = area.next_1();
            if old_area == area { break } // chaos has stabilized
        }

        area.total_occupied_count()
    }

    /// Runs the part 2 simulation
    fn simulation_2(&self) -> usize {
        let mut area = self.clone();
        loop {
            let old_area = area.clone();
            area = area.next_2();
            if old_area == area { break } // chaos has stabilized
        }

        area.total_occupied_count()
    }

    /// Returns the next evolution of this area according to the `adjacent_occupied_count`
    /// Answer for part 1
    fn next_1(&self) -> Area {
        let mut a = self.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let point = (x,y);
                let occ_count = self.adjacent_occupied_count(&point);

                a[point] = match (&self[point], occ_count) {
                    (Field::Seat, 0) => Field::Occupied,
                    (Field::Occupied, x) if x >= 4 => Field::Seat,
                    _ => continue,
                }
            }
        }
        a
    }

    /// Returns the next evolution of this area according to the `ray_occupied_count`
    /// Answer for part 2
    fn next_2(&self) -> Area {
        let mut a = self.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let point = (x,y);
                let occ_count = self.ray_occupied_count(&point);

                a[point] = match (&self[point], occ_count) {
                    (Field::Seat, 0) => Field::Occupied,
                    (Field::Occupied, x) if x >= 5 => Field::Seat,
                    _ => continue,
                }
            }
        }
        a
    }

    /// Returns if a point is within the bounds of this `Area`.
    /// Area can be safely indexed if this is true.
    fn in_bounds(&self, point: &(isize, isize)) -> bool {
        let (x, y) = point;

        *x >= 0 && *x < self.width as isize && *y >= 0 && *y < self.height as isize
    }

    /// Returns how many adjacent neighbouring fields are `Field::Occupied`
    fn adjacent_occupied_count(&self, point: &(usize, usize)) -> usize {
        let (x, y) = (point.0 as isize, point.1 as isize);
        [ // Neighboring fields
            (x-1, y-1), (x-1, y), (x-1, y+1),
            (x, y-1), (x, y+1),
            (x+1, y-1), (x+1, y), (x+1, y+1),
        ].iter()
            .filter(|&point| {
                if !self.in_bounds(point) { return false }

                self[(point.0 as usize, point.1 as usize)] == Field::Occupied
            })
            .count()
    }

    /// Returns how many ray traced seats are `Field::Occupied`
    fn ray_occupied_count(&self, point: &(usize, usize)) -> usize {
        [ // Directions to shoot rays in
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1), (0, 1),
            (1, -1), (1, 0), (1, 1),
        ].iter()
            .filter(|delta| {
                let (mut x, mut y) = (point.0 as isize, point.1 as isize);
                loop {
                    // extend ray by adding delta to acc
                    x += delta.0; y += delta.1;

                    // check if we've hit the edge of the area
                    if !self.in_bounds(&(x, y)) { return false };

                    // check current end of ray
                    match self[(x as usize, y as usize)] {
                        Field::Floor => continue,
                        Field::Occupied => return true,
                        Field::Seat => return false,
                    };
                }
            })
            .count()
    }

    /// Returns how many seats are occupied in the total area
    fn total_occupied_count(&self) -> usize {
        self.fields
            .iter()
            .filter(|&f| *f == Field::Occupied)
            .count()
    }
}

impl Index<(usize, usize)> for Area {
    type Output = Field;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 >= self.width || index.1 >= self.height {
            panic!("Area index out of range");
        }

        &self.fields[index.0 + index.1 * self.width]
    }
}

impl IndexMut<(usize, usize)> for Area {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.0 >= self.width || index.1 >= self.height {
            panic!("Area index out of range");
        }

        &mut self.fields[index.0 + index.1 * self.width]
    }
}

// For debugging. Unused in end result
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

    println!("{}", area.simulation_1());
    println!("{}", area.simulation_2());
}

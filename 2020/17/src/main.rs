use aoc_2020_common::common::load_file;
use std::ops::{Index, IndexMut};
#[macro_use]
extern crate itertools;


// PART 1: World3D

#[derive(Debug, Clone)]
struct World3D {
    length: u64,
    cubes: Vec<bool>,
}

impl World3D {
    fn new(length: u64) -> World3D {
        World3D {
            length,
            cubes: vec![false; (length * length * length) as usize],
        }
    }

    fn from(input: &str) -> World3D {
        let mut points: Vec<(u64,u64)> = Vec::new();

        for (x, l) in input.lines().enumerate() {
            for (y, c) in l.chars().enumerate() {
                if c == '#' {
                    points.push((x as u64, y as u64));
                }
            }
        }

        let mut world = World3D::new(32);

        let offset: u64 = 16;

        for (x, y) in points {
            world[(x + offset, y + offset, offset)] = true;
        }

        world
    }

    fn neighbor_count(&self, point: (u64, u64, u64)) -> u64 {
        let ipoint = (point.0 as i64, point.1 as i64, point.2 as i64);

        iproduct!(-1..=1, -1..=1, -1..=1)
            .filter(|p| p != &(0, 0, 0))
            .map(|(x, y, z): (i64, i64, i64) |
                self[(
                    (ipoint.0 + x) as u64,
                    (ipoint.1 + y) as u64,
                    (ipoint.2 + z) as u64)]
            )
            .filter(|b| *b)
            .count() as u64
    }

    fn next(&self) -> World3D {
        let mut next = self.clone();

        for point in iproduct!(1..(self.length-1), 1..(self.length-1), 1..(self.length-1)) {
            let neighbors = self.neighbor_count(point);
            let active = self[point];

            if active && (neighbors == 2 || neighbors == 3) {
               next[point] = true;
            } else if !active && neighbors == 3 {
               next[point] = true;
            } else {
               next[point] = false;
            }
        }

        next
    }

    fn active_count(&self) -> u64 {
        self.cubes.iter().filter(|&b| *b).count() as u64
    }
}

impl Index<(u64, u64, u64)> for World3D {
    type Output = bool;

    fn index(&self, index: (u64, u64, u64)) -> &Self::Output {
        let i = (index.0 + (index.1 * self.length) + (index.2 * self.length * self.length)) as usize;
        &self.cubes[i]
    }
}

impl IndexMut<(u64, u64, u64)> for World3D {
    fn index_mut(&mut self, index: (u64, u64, u64)) -> &mut Self::Output {
        let i = (index.0 + (index.1 * self.length) + (index.2 * self.length * self.length)) as usize;
        self.cubes.get_mut(i).unwrap()
    }
}

// PART 2: World4D

#[derive(Debug, Clone)]
struct World4D {
    length: u64,
    cubes: Vec<bool>,
}

impl World4D {
    fn new(length: u64) -> World4D {
        World4D {
            length,
            cubes: vec![false; (length * length * length * length) as usize],
        }
    }

    fn from(input: &str) -> World4D {
        let mut points: Vec<(u64,u64)> = Vec::new();

        for (x, l) in input.lines().enumerate() {
            for (y, c) in l.chars().enumerate() {
                if c == '#' {
                    points.push((x as u64, y as u64));
                }
            }
        }

        let mut world = World4D::new(32);

        let offset: u64 = 16;

        for (x, y) in points {
            world[(x + offset, y + offset, offset, offset)] = true;
        }

        world
    }

    fn neighbor_count(&self, point: (u64, u64, u64, u64)) -> u64 {
        let ipoint = (point.0 as i64, point.1 as i64, point.2 as i64, point.3 as i64);

        iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
            .filter(|p| p != &(0, 0, 0, 0))
            .map(|(x, y, z, w): (i64, i64, i64, i64) |
                self[(
                    (ipoint.0 + x) as u64,
                    (ipoint.1 + y) as u64,
                    (ipoint.2 + z) as u64,
                    (ipoint.3 + w) as u64)]
            )
            .filter(|b| *b)
            .count() as u64
    }

    fn next(&self) -> World4D {
        let mut next = self.clone();

        for point in iproduct!(1..(self.length-1), 1..(self.length-1), 1..(self.length-1), 1..(self.length-1)) {
            let neighbors = self.neighbor_count(point);
            let active = self[point];

            if active && (neighbors == 2 || neighbors == 3) {
                next[point] = true;
            } else if !active && neighbors == 3 {
                next[point] = true;
            } else {
                next[point] = false;
            }
        }

        next
    }

    fn active_count(&self) -> u64 {
        self.cubes.iter().filter(|&b| *b).count() as u64
    }
}

impl Index<(u64, u64, u64, u64)> for World4D {
    type Output = bool;

    fn index(&self, index: (u64, u64, u64, u64)) -> &Self::Output {
        &self.cubes[(index.0
            + (index.1 * self.length)
            + (index.2 * self.length * self.length)
            + (index.3 * self.length * self.length * self.length)) as usize]
    }
}

impl IndexMut<(u64, u64, u64, u64)> for World4D {
    fn index_mut(&mut self, index: (u64, u64, u64, u64)) -> &mut Self::Output {
        self.cubes.get_mut((index.0
            + (index.1 * self.length)
            + (index.2 * self.length * self.length)
            + (index.3 * self.length * self.length * self.length)) as usize).unwrap()
    }
}



fn main() {
    let input = load_file("./input/1.txt");
    let mut world3D = World3D::from(&input);
    let mut world4D = World4D::from(&input);

    for _ in 0..6 {
        world3D = world3D.next();
        world4D = world4D.next();
    }

    println!("{}", world3D.active_count());
    println!("{}", world4D.active_count());
}

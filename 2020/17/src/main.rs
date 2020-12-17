use aoc_2020_common::common::load_file;
use std::ops::{Index, IndexMut};
#[macro_use]
extern crate itertools;

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

    fn neighbor_count(&self, point: (u64, u64, u64)) -> u64 {
        let ipoint = (point.0 as i64, point.1 as i64, point.2 as i64);

        iproduct!(-1..=1, -1..=1, -1..=1)
            .filter(|t| t != &(0, 0, 0))
            .map(|(x, y, z): (i64, i64, i64) |
                self[((ipoint.0 + x) as u64, (ipoint.1 + y) as u64, (ipoint.2 + z) as u64)])
            .filter(|b| *b)
            .count() as u64
    }

    fn next(&mut self) {
        for x in 1..(self.length - 1) {
            for y in 1..(self.length - 1) {
                for z in 1..(self.length - 1) {
                    let point = (x, y, z);
                    let neighbors = self.neighbor_count(point);
                    if self[point] && neighbors == 2 || neighbors == 3 {
                        continue
                    } else if !self[point] && neighbors == 3 {
                        self[point] = true
                    }
                }
            }
        }
    }

    fn active_count(&self) -> u64 {
        self.cubes.iter().filter(|&b| *b).count() as u64
    }
}

impl Index<(u64, u64, u64)> for World3D {
    type Output = bool;

    fn index(&self, index: (u64, u64, u64)) -> &Self::Output {
        let i = (index.0 + index.1 * self.length + index.2 * self.length * self.length) as usize;
        &self.cubes[i]
    }
}

impl IndexMut<(u64, u64, u64)> for World3D {
    fn index_mut(&mut self, index: (u64, u64, u64)) -> &mut Self::Output {
        let i = (index.0 + index.1 * self.length + index.2 * self.length * self.length) as usize;
        self.cubes.get_mut(i).unwrap()
    }
}


fn main() {
    let input = load_file("./input/1.txt");
    println!("{}", input);

    let world = World3D::new(64);
}

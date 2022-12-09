use aoc_2022_common::challenge_input;
use nalgebra::DMatrix;

#[derive(Debug, Eq, PartialEq)]
struct HeightMap(Vec<Vec<u8>>);

#[derive(Debug, Eq, PartialEq)]
struct VisibilityMap(Vec<Vec<bool>>);

fn main() {
    let height_map = to_height_map(challenge_input());
    let visibility_map = height_map.map(|_| false);

    println!("{height_map}");
    println!("{visibility_map}");

    // println!("{}"height_map);
    // println!(visiblity_map.display());

    // let height = height_map.0.len();
    // let width = height_map.0[0].len();

    // // let height_map = HeightMap::from(challenge_input().as_ref());
    // // let mut visibility_map = VisibilityMap::from(&height_map);

    // // TODO: Cleanup by abstracting over scan direction

    // for y in 0..height {
    //     let mut current_height = None;
    //     for x in 0..width {
    //         let new_height = Some(height_map.0[y][x]);
    //         if new_height > current_height {
    //             visibility_map.0[y][x] = true;
    //             current_height = new_height;
    //         }
    //     }
    // }

    // for y in 0..height {
    //     let mut current_height = None;
    //     for x in (0..width).rev() {
    //         let new_height = Some(height_map.0[y][x]);
    //         if new_height > current_height {
    //             visibility_map.0[y][x] = true;
    //             current_height = new_height;
    //         }
    //     }
    // }

    // for x in 0..width {
    //     let mut current_height = None;
    //     for y in 0..height {
    //         let new_height = Some(height_map.0[y][x]);
    //         if new_height > current_height {
    //             visibility_map.0[y][x] = true;
    //             current_height = new_height;
    //         }
    //     }
    // }

    // for x in 0..width {
    //     let mut current_height = None;
    //     for y in (0..height).rev() {
    //         let new_height = Some(height_map.0[y][x]);
    //         if new_height > current_height {
    //             visibility_map.0[y][x] = true;
    //             current_height = new_height;
    //         }
    //     }
    // }

    // println!("{}", visibility_map.count());
}

fn to_height_map(input: String) -> DMatrix<u8> {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let height_data: Vec<u8> = lines
        .iter()
        .flat_map(|l| l.as_bytes().iter().map(|b| *b - 48).collect::<Vec<_>>())
        .collect();

    DMatrix::from_row_slice(rows, cols, &height_data)
}

impl VisibilityMap {
    pub fn count(&self) -> usize {
        self.0
            .iter()
            .map(|l| l.iter().filter(|v| **v).count())
            .sum()
    }
}

impl From<&HeightMap> for VisibilityMap {
    fn from(height_map: &HeightMap) -> Self {
        Self(
            height_map
                .0
                .iter()
                .map(|l| l.iter().map(|_| false).collect())
                .collect(),
        )
    }
}

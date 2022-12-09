use aoc_2022_common::challenge_input;

#[derive(Debug, Eq, PartialEq)]
struct HeightMap(Vec<Vec<u8>>);

#[derive(Debug, Eq, PartialEq)]
struct VisibilityMap(Vec<Vec<bool>>);

fn main() {
    let height_map = HeightMap::from(challenge_input().as_ref());
    let mut visibility_map = VisibilityMap::from(&height_map);

    let height = height_map.0.len();
    let width = height_map.0[0].len();

    // TODO: Cleanup by abstracting over scan direction

    for y in 0..height {
        let mut current_height = None;
        for x in 0..width {
            let new_height = Some(height_map.0[y][x]);
            if new_height > current_height {
                visibility_map.0[y][x] = true;
                current_height = new_height;
            }
        }
    }

    for y in 0..height {
        let mut current_height = None;
        for x in (0..width).rev() {
            let new_height = Some(height_map.0[y][x]);
            if new_height > current_height {
                visibility_map.0[y][x] = true;
                current_height = new_height;
            }
        }
    }

    for x in 0..width {
        let mut current_height = None;
        for y in 0..height {
            let new_height = Some(height_map.0[y][x]);
            if new_height > current_height {
                visibility_map.0[y][x] = true;
                current_height = new_height;
            }
        }
    }

    for x in 0..width {
        let mut current_height = None;
        for y in (0..height).rev() {
            let new_height = Some(height_map.0[y][x]);
            if new_height > current_height {
                visibility_map.0[y][x] = true;
                current_height = new_height;
            }
        }
    }

    println!("{}", visibility_map.count());
}

impl From<&str> for HeightMap {
    fn from(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|l| l.as_bytes().iter().map(|b| b - 48).collect())
                .collect(),
        )
    }
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

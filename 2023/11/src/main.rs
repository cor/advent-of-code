use aoc_2023_common::challenge_input;

#[derive(Clone)]
struct Universe(Vec<(usize, usize)>);
struct ExpandedUniverse<const E: usize>(Vec<(usize, usize)>);
type UniverseMatrix = Vec<Vec<bool>>;

impl From<UniverseMatrix> for Universe {
    fn from(universe: UniverseMatrix) -> Self {
        let mut galaxies = Vec::new();
        for (y, row) in universe.iter().enumerate() {
            for x in 0..row.len() {
                if universe[y][x] {
                    galaxies.push((x, y));
                }
            }
        }
        Self(galaxies)
    }
}

impl<const E: usize> From<Universe> for ExpandedUniverse<E> {
    fn from(universe: Universe) -> Self {
        let mut galaxies = universe.0.clone();
        let expansion = E - 1;

        let mut x = 0;
        while x < galaxies.iter().map(|g| g.0).max().unwrap() {
            if !galaxies.iter().any(|g| g.0 == x) {
                for galaxy in galaxies.iter_mut() {
                    if galaxy.0 > x {
                        galaxy.0 += expansion;
                    }
                }
                x += expansion
            }
            x += 1;
        }

        let mut y = 0;
        while y < galaxies.iter().map(|g| g.1).max().unwrap() {
            if !galaxies.iter().any(|g| g.1 == y) {
                for galaxy in galaxies.iter_mut() {
                    if galaxy.1 > y {
                        galaxy.1 += expansion;
                    }
                }
                y += expansion
            }
            y += 1;
        }

        ExpandedUniverse::<E>(galaxies)
    }
}

impl<const E: usize> ExpandedUniverse<E> {
    fn distances(&self) -> usize {
        self.0
            .iter()
            .flat_map(|a| {
                self.0
                    .iter()
                    .map(|b| manhattan_distance(a, b))
                    .collect::<Vec<_>>()
            })
            .sum::<usize>()
            / 2
    }
}

fn manhattan_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn parse(input: &str) -> UniverseMatrix {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b == b'#').collect())
        .collect()
}

fn main() {
    let input = challenge_input();
    let universe: Universe = parse(&input).into();

    let expanded: ExpandedUniverse<2> = universe.clone().into();
    println!("{}", expanded.distances());

    let expanded: ExpandedUniverse<1_000_000> = universe.into();
    println!("{}", expanded.distances());
}

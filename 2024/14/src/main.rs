use aoc_2024_common::challenge_input;
use nalgebra::Vector2;

const W: i16 = 11;
const H: i16 = 7;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Robot {
    position: Vector2<i16>,
    velocity: Vector2<i16>,
}

impl Robot {
    pub fn position_at(&self, second: i16) -> Vector2<i16> {
        let linear = self.position + (self.velocity * second);
        Vector2::new(linear.x.rem_euclid(W), linear.y.rem_euclid(H))
    }
}

fn draw_world(robots: &[Robot], second: i16) {
    for y in 0..H {
        for x in 0..W {
            let pos = Vector2::new(x, y);
            let count = robots
                .iter()
                .filter(|r| r.position_at(second) == pos)
                .count();
            match count {
                0 => print!("."),
                n => print!("{}", n),
            }
        }
        println!();
    }
    println!("s = {second}");
}

fn main() {
    let input = challenge_input();
    println!("{input}");
    let robots = input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(' ').unwrap();
            let (px, py) = p[2..].split_once(',').unwrap();
            let (vx, vy) = v[2..].split_once(',').unwrap();
            Robot {
                position: Vector2::new(px.parse().unwrap(), py.parse().unwrap()),
                velocity: Vector2::new(vx.parse().unwrap(), vy.parse().unwrap()),
            }
        })
        .collect::<Vec<_>>();

    draw_world(&robots, 100);

    // dbg!(robots);
}

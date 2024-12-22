use aoc_2024_common::challenge_input;
use nalgebra::Vector2;
use std::cmp::Ordering::*;

const W: i64 = 101;
const H: i64 = 103;
// const W: i64 = 11;
// const H: i64 = 7;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Robot {
    position: Vector2<i64>,
    velocity: Vector2<i64>,
}

impl Robot {
    pub fn position_at(&self, second: i64) -> Vector2<i64> {
        let linear = self.position + (self.velocity * second);
        Vector2::new(linear.x.rem_euclid(W), linear.y.rem_euclid(H))
    }
}

fn part_1(robots: &[Robot], seconds: i64) -> i64 {
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);

    for robot in robots {
        let pos = robot.position_at(seconds);

        match (pos.x.cmp(&(W / 2)), pos.y.cmp(&(H / 2))) {
            (Equal, _) | (_, Equal) => (),
            (Less, Less) => q1 += 1,
            (Less, Greater) => q2 += 1,
            (Greater, Less) => q3 += 1,
            (Greater, Greater) => q4 += 1,
        }
    }
    q1 * q2 * q3 * q4
}

/// Look for a straight line of at least 10 robots
fn part_2(robots: &[Robot]) -> i64 {
    (0..i64::MAX)
        .map(|second| {
            robots
                .iter()
                .map(|r| r.position_at(second))
                .collect::<Vec<_>>()
        })
        .position(|robots_at_second| {
            robots_at_second.iter().any(|robot| {
                (0..10).all(|line_x| {
                    robots_at_second.contains(&Vector2::new(robot.x + line_x, robot.y))
                })
            })
        })
        .unwrap() as i64
}

fn main() {
    let input = challenge_input();
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

    println!("{}", part_1(&robots, 100));
    draw_world(&robots, part_2(&robots));
}

fn draw_world(robots: &[Robot], second: i64) {
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
    println!("{second}");
}

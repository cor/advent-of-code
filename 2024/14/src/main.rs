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
    println!("s = {second}");
}

fn part_1(robots: &[Robot], seconds: i64) -> (i64, i64, i64, i64) {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

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
    // dbg!(q1);
    // dbg!(q2);
    // dbg!(q3);
    // dbg!(q4);

    (q1, q2, q3, q4)
}

// fn part_2(robots: &[Robot]) -> i64 {
//     for second in 0..i64::MAX {
//         let (q1, q2, q3, q4) = part_1(robots, second);

//         if q1 == q3 && q2 == q4 {
//             return second;
//             // break;
//         }
//     }
//     -1
// }

fn part_2(robots: &[Robot]) -> i64 {
    'outer: for second in 0..i64::MAX {
        let robots_at_second = robots
            .iter()
            .map(|r| r.position_at(second))
            .collect::<Vec<_>>();

        if second % 1_000_000 == 0 {
            println!("{}", second);
        }
        for robot in &robots_at_second {
            let symmetric_x = (W / 2) + ((W / 2) - robot.x);

            // println!("{} symmetric {}", robot.x, symmetric_x);

            if !robots_at_second.contains(&Vector2::new(symmetric_x, robot.y)) {
                continue 'outer;
            }
            //     match robot.x.cmp(&(W / 2)) {
            //     Less => robots_at_second.contains(&Vector2::new(((W / 2) - robot.x), robot.y)),
            //     Equal => (),
            //     Greater => todo!(),
            // };

            // if ! {
            //     continue;
            // }
        }
        return second;
    }
    -100

    // for robot in robots {
    //     let pos = robot.position_at(100);

    //     match (pos.x.cmp(&(W / 2)), pos.y.cmp(&(H / 2))) {
    //         (Equal, _) | (_, Equal) => (),
    //         (Less, Less) => q1 += 1,
    //         (Less, Greater) => q2 += 1,
    //         (Greater, Less) => q3 += 1,
    //         (Greater, Greater) => q4 += 1,
    //     }
    // }

    // q1 * q2 * q3 * q4
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
    let (q1, q2, q3, q4) = part_1(&robots, 100);
    println!("{}", q1 * q2 * q3 * q4);

    let p2 = part_2(&robots);
    draw_world(&robots, p2);
    println!("{}", p2);
}

#![feature(destructuring_assignment)]
use std::num::ParseIntError;
use std::str::FromStr;

enum Direction {
    North,
    East,
    South,
    West,
    Right,
    Left,
    Forward,
}

struct Instruction {
    direction: Direction,
    value: isize,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction: Direction = match s.chars().next().unwrap() {
            'N' => Direction::North,
            'E' => Direction::East,
            'S' => Direction::South,
            'W' => Direction::West,
            'L' => Direction::Left,
            'R' => Direction::Right,
            'F' => Direction::Forward,
            _ => unreachable!(),
        };

        let value = s.chars().skip(1).collect::<String>().parse::<usize>()? as isize;

        Ok(Instruction { direction, value })
    }
}

fn main() {
    let instructions: Vec<Instruction> = include_str!("input")
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect();

    let mut ns_wp: isize = 1; // +North/-South
    let mut ew_wp: isize = 10; // +East/-West
    let mut ns_sp: isize = 0; // ship
    let mut ew_sp: isize = 0;

    for ins in instructions {
        println!(
            "Ship: {}, {}\nWaypoint: {}, {}\n",
            ew_sp, ns_sp, ew_wp, ns_wp
        );
        match ins.direction {
            Direction::North => ns_wp += ins.value,
            Direction::South => ns_wp -= ins.value,
            Direction::East => ew_wp += ins.value,
            Direction::West => ew_wp -= ins.value,
            Direction::Right => {
                // println!("{} ew {} ns, rotating {}° CW", ew_wp, ns_wp, ins.value);
                for _ in 0..(ins.value / 90) {
                    (ew_wp, ns_wp) = (ns_wp, -ew_wp);
                }
                // println!("{} ew {} ns", ew_wp, ns_wp);
            }
            Direction::Left => {
                // println!("{} ew {} ns, rotating {}° CCW", ew_wp, ns_wp, ins.value);
                for _ in 0..(ins.value / 90) {
                    (ew_wp, ns_wp) = (-ns_wp, ew_wp);
                }
                // println!("{} ew {} ns", ew_wp, ns_wp);
            }
            Direction::Forward => {
                ns_sp += ns_wp * ins.value;
                ew_sp += ew_wp * ins.value;
            }
        }
    }
    println!(
        "Ship: {}, {}\nWaypoint: {}, {}\n",
        ew_sp, ns_sp, ew_wp, ns_wp
    );
    dbg!(ns_sp.abs() + ew_sp.abs());
}

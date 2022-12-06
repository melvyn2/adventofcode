extern crate core;

use crate::RPC::{Paper, Rock, Scissors};

#[derive(Copy, Clone, Debug)]
enum RPC {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
impl RPC {
    fn as_int(&self) -> u8 {
        *self as u8
    }

    fn from_int(x: u8) -> Self {
        match x {
            1 => Rock,
            2 => Paper,
            3 => Scissors,
            0 => Scissors, // Stupid hack
            _ => panic!(),
        }
    }

    fn offset(&self, o: i8) -> Self {
        Self::from_int((((self.as_int() as i8) + o) % 3) as u8)
    }

    fn from_u8_char(c: u8) -> Self {
        if c > 87 {
            Self::from_int(c - 87)
        } else {
            Self::from_int(c - 64)
        }
    }

    fn from_pair_pt2(o: u8, g: u8) -> (Self, Self) {
        let opp = Self::from_u8_char(o);
        let you = match g {
            88 => opp.offset(-1), // X = Lose
            89 => opp,            // Y = Tie
            90 => opp.offset(1),  // Z = Win
            _ => panic!(),
        };
        (opp, you)
    }

    fn score(&self, other: &Self) -> u32 {
        self.as_int() as u32
            + match (self, other) {
                (Rock, Rock) => 3,
                (Paper, Paper) => 3,
                (Scissors, Scissors) => 3,
                (Rock, Scissors) => 6,
                (Paper, Rock) => 6,
                (Scissors, Paper) => 6,
                (Rock, Paper) => 0,
                (Paper, Scissors) => 0,
                (Scissors, Rock) => 0,
            }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut rounds: Vec<(RPC, RPC)> = vec![];

    for line in input.lines() {
        let mut inst_pair = line.bytes();
        rounds.push(RPC::from_pair_pt2(
            inst_pair.next().unwrap(),
            inst_pair.nth(1).unwrap(),
        ));
    }

    dbg!(&rounds);

    dbg!(rounds.iter().map(|(opp, you)| you.score(opp)).sum::<u32>());
}

extern crate core;

use crate::RPC::{Paper, Rock, Scissors};

#[derive(Copy, Clone)]
enum RPC {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
impl RPC {
    fn as_int(&self) -> u32 {
        *self as u32
    }
    fn from_int(x: u8) -> Self {
        match x {
            1 => Rock,
            2 => Paper,
            3 => Scissors,
            _ => panic!(),
        }
    }
    // fn offset(&self, o: u8) -> Self {
    //     (self.as_int() + o) % 3 + 1
    // }
    fn from_u8_char(c: u8) -> Self {
        if c > 87 {
            Self::from_int(c - 87)
        } else {
            Self::from_int(c - 64)
        }
    }
    // fn from_pair(o: &str, g: &str) -> (Self, Self) {
    //     let opp = match o {
    //         "A" => Rock,
    //         "B" => Paper,
    //         "C" => Scissors,
    //
    //         _ => panic!(),
    //     };
    //     let you = match g {
    //         "X" => {}
    //         "Y" => {}
    //         "Z" => {}
    //         _ => panic!(),
    //     };
    // }
    fn score(&self, other: &Self) -> u32 {
        self.as_int()
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

        let opp = RPC::from_u8_char(inst_pair.next().unwrap());
        let you = RPC::from_u8_char(inst_pair.nth(1).unwrap());
        rounds.push((opp, you));
    }

    dbg!(rounds.iter().map(|(opp, you)| you.score(opp)).sum::<u32>());
}

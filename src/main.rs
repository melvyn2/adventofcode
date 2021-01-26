extern crate ring_algorithm;
use ring_algorithm::chinese_remainder_theorem;

fn main() {
    let mut lines = include_str!("input").lines();
    let (idx, buses): (Vec<isize>, Vec<isize>) = lines
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, x)| *x != "x")
        .map(|(i, s)| (i as isize, s.parse::<isize>().unwrap()))
        .unzip();

    dbg!(chinese_remainder_theorem(&idx, &buses));
}

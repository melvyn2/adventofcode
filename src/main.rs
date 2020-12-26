use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let answer: usize = include_str!("input")
        .split("\n\n")
        .map(|group| {
            group
            .lines()
            .map(|person| HashSet::<u8>::from_iter(person.as_bytes().iter().copied()))
            .fold(HashSet::from_iter("abcdefghijklmnopqrstuvwxyz".to_string().bytes().into_iter()), |acc, p| {
                acc.intersection(&p).cloned().collect()
            })
            .len()
        })
        .sum();

    dbg!(answer);
}

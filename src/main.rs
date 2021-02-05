use std::ops::RangeInclusive;

// Can't create impl for foreign type (E0117)
fn range_from_str(s: &str) -> RangeInclusive<usize> {
    let mut split = s.split('-');
    let lhs = split.next().unwrap().parse::<usize>().unwrap();
    let rhs = split.next().unwrap().parse::<usize>().unwrap();
    lhs..=rhs
}

fn main() {
    let mut input = include_str!("input").split("\n\n");
    let constraints: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> = input
        .next()
        .unwrap()
        .lines()
        .map(|s| {
            let mut split = s.split(' ').rev();
            let r2: RangeInclusive<usize> = range_from_str(split.next().unwrap());
            let r1: RangeInclusive<usize> = range_from_str(split.nth(1).unwrap());
            (r1, r2)
        })
        .collect();
    let self_ticket: Vec<usize> = input
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let other_tickets: Vec<Vec<usize>> = input
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|x| {
            x.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    let mut erate: Vec<usize> = vec![];
    'val: for val in other_tickets.iter().flatten() {
        for constraint_pair in &constraints {
            if constraint_pair.0.contains(val) {
                continue 'val;
            }
            if constraint_pair.1.contains(val) {
                continue 'val;
            }
        }
        erate.push(*val);
    }
    dbg!(erate.iter().sum::<usize>());
}

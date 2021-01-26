use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("input").lines();
    let ts: usize = lines.next().unwrap().parse().unwrap();
    let buses: HashMap<usize, usize> = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|&x| x != "x")
        .map(|s| {
            let id = s.parse::<usize>().unwrap();
            let mut cycle = id;
            while cycle < ts {
                cycle += id;
            }
            (cycle, id)
        })
        .collect();
    dbg!(buses.get(buses.keys().min().unwrap()).unwrap() * (buses.keys().min().unwrap() - ts));
}

extern crate rayon;
use rayon::prelude::*;

fn main() {
    let mut lines = include_str!("input").lines();
    let buses: Vec<usize> = lines
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap_or(0))
        .collect();

    let max_bus: &usize = buses.iter().max().unwrap();
    let max_bus_offset: usize = buses.iter().position(|x| x == max_bus).unwrap();

    let res: usize = (1..10000000000000000usize)
        .into_par_iter()
        .find_first(|x| {
            let ts_try = (x * max_bus) - max_bus_offset;
            let mut is_valid = true;
            for (idx, bus) in buses.iter().enumerate() {
                if *bus != 0 && (ts_try + idx) % bus != 0 {
                    is_valid = false;
                    break;
                }
            }
            is_valid
        })
        .unwrap();
    dbg!((res * max_bus) - max_bus_offset);
}

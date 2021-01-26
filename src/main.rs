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

    'outer: for ts_mult in 10000000000..10000000000000000 {
        print!("\r{}", ts_mult as f64 / 10000000000000000f64);
        let ts_try = (ts_mult * max_bus) - max_bus_offset;
        for (idx, bus) in buses.iter().enumerate() {
            if *bus != 0 && (ts_try + idx) % bus != 0 {
                continue 'outer;
            }
        }
        dbg!(buses);
        dbg!(ts_try);
        break;
    }
}

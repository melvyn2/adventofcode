fn main() {
    let mut adapters: Vec<usize> = include_str!("input")
        .split('\n')
        .map( |num| num.parse::<usize>().unwrap() )
        .collect();
    adapters.sort_unstable();
    let adapters = adapters;

    let device_rating: usize = adapters.iter().max().unwrap() + 3;
    dbg!(&adapters, device_rating);

    let mut ch1: usize = 0;
    let mut ch3: usize = 0;
    for (idx, adapter) in adapters.iter().enumerate() {
        if idx == 0 { continue }
        let change = adapter - adapters[idx-1];
        if change == 3 {
            ch3 += 1;
        } else if change == 1 {
            ch1 += 1;
        } else {
            panic!(change);
        }
    }
    dbg!(ch1, ch3, (ch1 + 1) * (ch3 + 1));
}

use std::ops::Range;

fn main() {
    let input = include_str!("input.txt");

    let mut sec = input.split("\n\n");

    let seeds = sec
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|s| s[0]..s[0] + s[1])
        .collect::<Vec<Range<u64>>>();

    let maps = sec
        .map(|m| {
            m.lines()
                .skip(1)
                .map(|l| {
                    let v = l
                        .split(' ')
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                    (v[1]..v[1] + v[2], (v[0] as i64 - v[1] as i64))
                })
                .collect::<Vec<(Range<u64>, i64)>>()
        })
        .collect::<Vec<Vec<(Range<u64>, i64)>>>();

    let res = seeds
        .into_iter()
        .map(|seed_range| {
            seed_range
                .map(|s| {
                    let mut c = s as i64;
                    for map in &maps {
                        let offset = map
                            .iter()
                            .find_map(|(r, o)| {
                                if r.contains(&(c as u64)) {
                                    Some(*o)
                                } else {
                                    None
                                }
                            })
                            .unwrap_or(0);
                        c += offset;
                    }
                    c
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    dbg!(res);
}

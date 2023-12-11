use std::collections::BTreeMap;
use std::process::exit;

fn main() {
    let input = include_str!("input.txt");

    let (dir_str, map_str) = input.split_once("\n\n").unwrap();
    let directions = dir_str
        .bytes()
        .map(|b| match b {
            b'L' => 0,
            b'R' => 1,
            _ => unreachable!(),
        })
        .collect::<Vec<usize>>();

    let map = map_str
        .lines()
        .map(|s| {
            let (key, vals_str) = s.split_once(" = ").unwrap();
            let vals = vals_str[1..vals_str.len() - 1].split_once(", ").unwrap();
            (
                key.as_bytes().try_into().unwrap(),
                [
                    vals.0.as_bytes().try_into().unwrap(),
                    vals.1.as_bytes().try_into().unwrap(),
                ],
            )
        })
        .collect::<BTreeMap<[u8; 3], [[u8; 3]; 2]>>();

    let mut pos = b"AAA";
    let mut step = 0;
    while pos != b"ZZZ" {
        let dir = directions[step % directions.len()];
        pos = &map[pos][dir];
        step += 1;
    }

    dbg!(step);
}

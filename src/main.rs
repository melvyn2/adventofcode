#![feature(let_chains)]
#![feature(iter_map_windows)]

fn main() {
    let input = include_str!("input.txt");

    let value_histories = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut acc = 0;
    for hist in value_histories {
        let mut inner = 0;
        let mut coeff = 1;
        let mut diffs = hist.clone();

        while diffs.iter().any(|&x| x != 0) {
            inner += coeff * diffs[0];
            coeff *= -1;
            let new = diffs
                .iter()
                .map_windows(|[&x, &y]| y - x)
                .collect::<Vec<i64>>();
            diffs = new;
        }

        dbg!(inner);
        acc += inner;
    }
    dbg!(acc);
}

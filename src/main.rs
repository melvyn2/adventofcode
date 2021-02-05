fn main() {
    let mut spoken: Vec<usize> = vec![11, 0, 1, 10, 5, 19];
    for idx in (&spoken).len()..2020 {
        let prev = spoken.clone();
        let a: Vec<(usize, &usize)> = prev
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, &x)| x == prev[idx - 1])
            .collect();
        if a.len() > 1usize {
            spoken.push(a[0].0 - a[1].0);
        } else {
            spoken.push(0);
        }
    }
    dbg!(spoken[2020 - 1]);
}

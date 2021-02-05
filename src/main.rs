fn main() {
    let target: f64 = 30000000f64;
    let mut spoken: Vec<usize> = vec![0, 3, 6];
    for idx in (&spoken).len()..target as usize {
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
        print!("\r {}", idx as f64 / target)
    }
    dbg!(spoken[target as usize - 1]);
}

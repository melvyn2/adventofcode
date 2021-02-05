fn main() {
    let target: f64 = 30000f64;
    let mut spoken: Vec<usize> = vec![0, 3, 6];
    for idx in (&spoken).len()..target as usize {
        let mut a = spoken
            .iter()
            .enumerate()
            .rev()
            .skip(1)
            .filter(|(_, &x)| x == spoken[idx - 1]);
        let n = a.next();
        let res: usize = match n {
            Some((second_idx, _)) => (idx - 1) - second_idx,
            None => 0,
        };
        spoken.push(res);
        // print!("\r {}", (idx as f64 / target));
    }
    println!();
    // dbg!(&spoken);
    dbg!(spoken[target as usize - 1]);
}

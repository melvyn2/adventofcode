use std::collections::HashMap;

fn main() {
    let target: usize = 30000000;
    let input: Vec<usize> = vec![11, 0, 1, 10, 5, 19];
    let (input, last) = input.split_at(input.len() - 1);
    let mut last = *last.first().unwrap();
    let mut spoken: HashMap<usize, usize> = input
        .iter()
        .enumerate()
        .map(|(i, x)| (*x as usize, i))
        .collect();
    for idx in (&spoken).len()..target - 1 {
        let res = match spoken.get(&last) {
            Some(second_last_idx) => idx - second_last_idx,
            None => 0,
        };
        spoken.insert(last, idx);
        last = res;
    }
    dbg!(last);
}

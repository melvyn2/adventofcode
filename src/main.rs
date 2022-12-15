use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let mut knots = vec![(0, 0); 10];
    let mut pos_set: HashSet<(i32, i32)> = HashSet::from([knots[9]]);
    for (dir_s, num_s) in input.lines().map(|s| s.split_once(' ').unwrap()) {
        let num = num_s.parse::<i32>().unwrap();
        println!("{}  {}", dir_s, num);
        for _ in 0..num {
            match dir_s.chars().next().unwrap() {
                'L' => {
                    knots[0].0 -= 1;
                }
                'R' => {
                    knots[0].0 += 1;
                }
                'U' => {
                    knots[0].1 -= 1;
                }
                'D' => {
                    knots[0].1 += 1;
                }
                _ => unreachable!(),
            }
            for head_idx in 0..9 {
                let mut tail = knots[head_idx + 1];
                let diff = (knots[head_idx].0 - tail.0, knots[head_idx].1 - tail.1);
                if diff.0.abs() + diff.1.abs() > 2 {
                    tail.0 += diff.0 / diff.0.abs();
                    tail.1 += diff.1 / diff.1.abs();
                } else if diff.0.abs() > 1 {
                    tail.0 += diff.0 / diff.0.abs();
                } else if diff.1.abs() > 1 {
                    tail.1 += diff.1 / diff.1.abs();
                }
                knots[head_idx + 1] = tail;
            }
            pos_set.insert(knots[9]);
        }
    }
    dbg!(pos_set.len());
}

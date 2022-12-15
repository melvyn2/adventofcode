use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut pos_set: HashSet<(i32, i32)> = HashSet::from([tail]);
    for (dir_s, num_s) in input.lines().map(|s| s.split_once(' ').unwrap()) {
        let num = num_s.parse::<i32>().unwrap();
        println!("{}  {}", dir_s, num);
        for _ in 0..num {
            match dir_s.chars().next().unwrap() {
                'L' => {
                    head.0 -= 1;
                }
                'R' => {
                    head.0 += 1;
                }
                'U' => {
                    head.1 -= 1;
                }
                'D' => {
                    head.1 += 1;
                }
                _ => unreachable!(),
            }
            let diff = (head.0 - tail.0, head.1 - tail.1);
            if diff.0.abs() + diff.1.abs() > 2 {
                tail.0 += diff.0 / diff.0.abs();
                tail.1 += diff.1 / diff.1.abs();
            } else if diff.0.abs() > 1 {
                tail.0 += diff.0 / diff.0.abs();
            } else if diff.1.abs() > 1 {
                tail.1 += diff.1 / diff.1.abs();
            }
            pos_set.insert(tail);
        }
        for yp in -5..=5 {
            for xp in -5..=5 {
                if (xp, yp) == head {
                    print!("H");
                } else if (xp, yp) == tail {
                    print!("T")
                } else if (xp, yp) == (0, 0) {
                    print!("s");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
    dbg!(pos_set.len());
}

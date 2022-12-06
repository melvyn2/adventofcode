fn main() {
    let input = include_str!("input.txt");
    let items: Vec<char> = input
        .lines()
        .map(|line| {
            let (side1, side2) = line.split_at(line.len() / 2);
            for c1 in side1.chars() {
                for c2 in side2.chars() {
                    if c1 == c2 {
                        return c1;
                    }
                }
            }
            unreachable!()
        })
        .collect();
    let priorities: Vec<u32> = items
        .iter()
        .map(|&c| {
            if c.is_lowercase() {
                (c as u32) - 96
            } else {
                (c as u32) - 38
            }
        })
        .collect();
    dbg!(&items);
    dbg!(&priorities);
    dbg!(priorities.iter().sum::<u32>());
}

fn main() {
    let input = include_str!("input.txt");
    let items: Vec<char> = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks_exact(3)
        .map(|lines| {
            for c1 in lines[0].chars() {
                for c2 in lines[1].chars() {
                    for c3 in lines[2].chars() {
                        // Lol
                        if (c1 == c2) && (c2 == c3) {
                            return c1;
                        }
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

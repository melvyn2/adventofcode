use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    for c in input
        .chars()
        .enumerate()
        .collect::<Vec<(usize, char)>>()
        .windows(14)
    {
        if c.iter()
            .cloned()
            .map(|(_, c)| c)
            .collect::<HashSet<char>>()
            .len()
            == c.len()
        {
            dbg!(c.last().unwrap().0 + 1);
            break;
        }
    }
}

fn main() {
    let mut input_s = include_str!("input.txt").split("\n\n");
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9]; // Set manually
    for line in input_s.next().unwrap().lines() {
        for (idx, c) in line
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_ascii_alphabetic())
        {
            stacks[idx / 4].insert(0, c);
        }
    }
    let mut moves: Vec<(usize, usize, usize)> = vec![];
    for line in input_s.next().unwrap().lines() {
        let mut words = line.split_whitespace();
        let ct = words.nth(1).unwrap().parse::<usize>().unwrap();
        let src = words.nth(1).unwrap().parse::<usize>().unwrap();
        let dest = words.nth(1).unwrap().parse::<usize>().unwrap();
        moves.push((src, dest, ct));
    }
    for (src, dest, ct) in moves {
        let src_range = stacks[src - 1].len() - ct..stacks[src - 1].len();
        let mut itms = stacks[src - 1].drain(src_range).collect();
        stacks[dest - 1].append(&mut itms);
    }
    dbg!(stacks
        .iter()
        .map(|s| *s.last().unwrap())
        .collect::<String>());
}

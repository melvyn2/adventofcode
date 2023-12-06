fn main() {
    let input = include_str!("input.txt");
    let acc: u32 = input
        .lines()
        .map(|l| {
            let mut nums = l.chars().filter(|c| c.is_numeric());
            let first = nums.next().and_then(|c| c.to_digit(10)).unwrap();
            first * 10 + nums.last().and_then(|c| c.to_digit(10)).unwrap_or(first)
        })
        .sum();
    dbg!(acc);
}

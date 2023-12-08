fn main() {
    let input = include_str!("input.txt");

    let acc = input
        .lines()
        .map(|l| {
            let nums = l.split_once(": ").unwrap().1;
            let (wins_str, given_str) = nums.split_once(" | ").unwrap();
            let wins = wins_str
                .split(' ')
                .filter(|&s| !s.is_empty())
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>();
            let given = given_str
                .split(' ')
                .filter(|&s| !s.is_empty())
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>();
            let count = given.iter().filter(|&n| wins.contains(n)).count();
            if count == 0 {
                0
            } else {
                2u32.pow(count as u32 - 1)
            }
        })
        .sum::<u32>();
    dbg!(acc);
}

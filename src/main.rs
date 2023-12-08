fn main() {
    let input = include_str!("input.txt");

    let map = input
        .lines()
        .map(|l| {
            let (_game_str, nums) = l.split_once(": ").unwrap();

            let (wins_str, given_str) = nums.split_once(" | ").unwrap();
            let winning = wins_str
                .split(' ')
                .filter(|&s| !s.is_empty())
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>();
            let given = given_str
                .split(' ')
                .filter(|&s| !s.is_empty())
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>();
            let wins = given.iter().filter(|&n| winning.contains(n)).count();
            wins
        })
        .collect::<Vec<usize>>();

    let mut acc = 0;
    let mut cur = vec![1; map.len()];
    let mut next = vec![0; map.len()];

    while cur.iter().sum::<usize>() > 0 {
        for (game, &wins) in map.iter().enumerate() {
            acc += cur[game];
            for i in next.iter_mut().skip(game + 1).take(wins) {
                *i += cur[game];
            }
        }
        cur = next;
        next = vec![0; map.len()];
    }

    dbg!(acc);
}

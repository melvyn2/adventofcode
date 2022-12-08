fn main() {
    let input = include_str!("input.txt");
    let trees: Vec<Vec<u8>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();
    let res = trees
        .clone()
        .iter()
        .enumerate()
        .map(|(r_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(c_idx, tree)| {
                    if ((r_idx == 0 || r_idx == trees.len() - 1)
                        || (c_idx == 0 || c_idx == row.len() - 1))
                        || ((tree > row[..c_idx].iter().max().unwrap())
                            || (tree > row[c_idx + 1..].iter().max().unwrap())
                            || (tree > &trees[..r_idx].iter().map(|r| r[c_idx]).max().unwrap())
                            || (tree > &trees[r_idx + 1..].iter().map(|r| r[c_idx]).max().unwrap()))
                    {
                        // dbg!(tree);
                        return 1;
                    }
                    return 0;
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    for row in &res {
        println!("{:?}", row);
    }
    dbg!(res.iter().flatten().sum::<u32>());
}

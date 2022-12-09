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
                    if (r_idx == 0 || r_idx == trees.len() - 1)
                        || (c_idx == 0 || c_idx == row.len() - 1)
                    {
                        return 0;
                    }
                    let r_vd = row[c_idx + 1..]
                        .iter()
                        .enumerate()
                        .find(|(_, t)| t >= &tree)
                        .unwrap_or((row[c_idx + 1..].len() - 1, &0))
                        .0
                        + 1;
                    let l_vd = row[..c_idx]
                        .iter()
                        .rev()
                        .enumerate()
                        .find(|(_, t)| t >= &tree)
                        .unwrap_or((row[..c_idx].len() - 1, &0))
                        .0
                        + 1;
                    let col = trees.iter().map(|r| r[c_idx]).collect::<Vec<u8>>();
                    let u_vd = col[..r_idx]
                        .iter()
                        .rev()
                        .enumerate()
                        .find(|(_, t)| t >= &tree)
                        .unwrap_or((col[..r_idx].len() - 1, &0))
                        .0
                        + 1;
                    let d_vd = col[r_idx + 1..]
                        .iter()
                        .enumerate()
                        .find(|(_, t)| t >= &tree)
                        .unwrap_or((col[r_idx + 1..].len() - 1, &0))
                        .0
                        + 1;

                    l_vd * r_vd * d_vd * u_vd
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    for row in &res {
        println!("{:?}", row);
    }

    dbg!(res.iter().flatten().max());
}

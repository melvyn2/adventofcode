
fn main() {
    let input: Vec<String> = include_str!("input")
        .split('\n')
        .map( |s| s.to_string())
        .collect();

    let mut sid_vec: Vec<u16> = vec!();
    let mut max_sid = 0;

    for seat in input {
        let (rowi, coli) = seat.split_at(7);

        let mut row: u16 = 0;
        for b in rowi.chars() {
            match b {
                'F' => {
                    row = row << 1;
                }
                'B' => {
                    row = (row << 1) | 1;
                }
                _ => ()
            }
        }

        let mut col: u16 = 0;
        for b in coli.chars() {
            match b {
                'L' => {
                    col = col << 1;
                }
                'R' => {
                    col = (col << 1) | 1;
                }
                _ => ()
            };
        }

        let sid = (row * 8) + col;
        sid_vec.push(sid);
        if sid > max_sid {
            max_sid = sid;
        };
    }

    // dbg!(sid_vec);

    for sid in &sid_vec {
        if sid_vec.contains(&(sid + 2)) && ! (sid_vec.contains(&(sid + 1))) {
            dbg!(sid + 1);
        }
    }
}

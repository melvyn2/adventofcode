fn main() {
    let input = include_str!("input.txt");
    let mut reg_x: i32 = 1;
    let mut cyc: u32 = 0;
    let mut crt: Vec<Vec<char>> = vec![vec![' '; 40]; 6];
    for inst_str in input.lines() {
        let crt_x: usize = (cyc % 40) as usize;
        let crt_y: usize = (cyc / 40) as usize;
        if (reg_x - 1..=reg_x + 1).contains(&(crt_x as i32)) {
            crt[crt_y][crt_x] = '#';
        }
        cyc += 1;
        if let Some((_, add_s)) = inst_str.split_once(' ') {
            let crt_x: usize = (cyc % 40) as usize;
            let crt_y: usize = (cyc / 40) as usize;
            if (reg_x - 1..=reg_x + 1).contains(&(crt_x as i32)) {
                crt[crt_y][crt_x] = '#';
            }
            cyc += 1;
            reg_x += add_s.parse::<i32>().unwrap();
        };
    }
    for line in crt {
        println!("{}", line.iter().collect::<String>());
    }
}

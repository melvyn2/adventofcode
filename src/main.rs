fn main() {
    let input = include_str!("input.txt");
    let mut reg_x: i32 = 1;
    let mut cyc: u32 = 0;
    let mut acc: u32 = 0;
    for inst_str in input.lines() {
        cyc += 1;
        if cyc % 40 == 20 {
            acc += cyc * reg_x as u32;
        }
        if let Some((_, add_s)) = inst_str.split_once(' ') {
            cyc += 1;
            if cyc % 40 == 20 {
                acc += cyc * reg_x as u32;
            }
            reg_x += add_s.parse::<i32>().unwrap();
        };
    }
    dbg!(acc);
}

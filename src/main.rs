fn main() {
    let u36_max: u64 = 68719476735;
    let ins: Vec<(usize, &str)> = include_str!("input")
        .lines()
        .map(|s| {
            let val = s.split("= ").nth(1).unwrap();

            let split = s.split_at(4);
            let addr = if split.0 == "mem[" {
                assert!(val.parse::<u64>().unwrap() <= u36_max);
                split.1.split(']').next().unwrap().parse::<usize>().unwrap()
            } else {
                assert_eq!(val.len(), 36);
                0usize
            };
            (addr, val)
        })
        .collect();
    let mut sys_mem: Vec<u64> = vec![0u64; ins.iter().map(|x| x.0).max().unwrap() + 1];
    let mut global_mask: &str = "";
    for instruction in ins {
        if instruction.0 == 0 {
            global_mask = instruction.1
        } else {
            let mut val: u64 = instruction.1.parse().unwrap();
            for char in global_mask.chars().enumerate() {
                match char.1 {
                    'X' => continue,
                    '1' => val |= 1 << (35 - char.0),
                    '0' => val &= !(1 << (35 - char.0)),
                    _ => unreachable!(),
                }
            }
            sys_mem[instruction.0] = val;
        }
    }
    dbg!(sys_mem.iter().sum::<u64>());
}

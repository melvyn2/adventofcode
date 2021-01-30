use std::collections::HashMap;

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
    let mut sys_mem: HashMap<usize, u64> = HashMap::new();
    let mut global_mask = "".chars().rev().enumerate();
    for instruction in ins {
        if instruction.0 == 0 {
            global_mask = instruction.1.chars().rev().enumerate();
        } else {
            let mut addrs: Vec<usize> = vec![instruction.0];
            for index in global_mask
                .clone()
                .filter(|(_, x)| *x == '1')
                .map(|(idx, _)| idx)
            {
                addrs[0] |= 1 << index;
            }
            for index in global_mask
                .clone()
                .filter(|(_, x)| *x == 'X')
                .map(|(idx, _)| idx)
            {
                for addr in addrs.clone() {
                    addrs.push(addr ^ (1 << index))
                }
            }

            sys_mem.extend(
                addrs
                    .iter()
                    .map(|a| (*a, instruction.1.parse::<u64>().unwrap())),
            );
        }
    }
    dbg!(sys_mem.values().sum::<u64>());
}

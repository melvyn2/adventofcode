fn main() {
    let numbers: Vec<usize> = include_str!("input")
        .split('\n')
        .map( |num| num.parse::<usize>().unwrap() )
        .collect();

    // for idx in 0..numbers.len()-25 {
    //     let preamble:  &[usize] = &numbers[idx..idx+25];
    //     let num: usize = numbers[idx+25];
    //     assert_eq!(&preamble.len(), &25usize);
    //     let mut is_sum: bool = false;
    //     'first: for first in preamble.iter() {
    //         for second in preamble.iter() {
    //             if first == second {
    //                 continue
    //             }
    //             if num == first + second {
    //                 is_sum = true;
    //                 break 'first
    //             }
    //         }
    //     }
    //     if !is_sum {
    //         dbg!(num);
    //     }
    // }
    let search: usize = 177777905;
    'outer: for length in 2..100 {
        for idx in 0..numbers.len()-length {
            let test: &[usize] = &numbers[idx..idx + length];
            assert_eq!(test.len(), length);
            if test.iter().sum::<usize>() == search {
                dbg!(test.iter().min().unwrap() + test.iter().max().unwrap());
                break 'outer
            }
        }
    }
}

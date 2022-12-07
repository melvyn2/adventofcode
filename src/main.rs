fn main() {
    let input = include_str!("input.txt");
    let items: u32 = input
        .lines()
        .map(|line| {
            let mut split = line.split(',');
            let (mut r1s, mut r2s) = (
                split.next().unwrap().split('-'),
                split.next().unwrap().split('-'),
            );
            let r1 = r1s.next().unwrap().parse::<u8>().unwrap()
                ..=r1s.next().unwrap().parse::<u8>().unwrap();
            let r2 = r2s.next().unwrap().parse::<u8>().unwrap()
                ..=r2s.next().unwrap().parse::<u8>().unwrap();

            (r1.contains(r2.start()) || r1.contains(r2.end()))
                || (r2.contains(r1.start()) || r2.contains(r1.end()))
        })
        .map(|b| b as u32)
        .sum();
    dbg!(items);
}

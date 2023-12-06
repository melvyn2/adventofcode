fn str_to_digits(s: &[u8]) -> u32 {
    let digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .map(|s| s.as_bytes());

    let mut first: u32 = 11;
    'outer: for i in 0..s.len() {
        if (s[i] as char).is_numeric() {
            first = (s[i] as char).to_digit(10).unwrap();
            break 'outer;
        }
        for (n, &d) in digits.iter().enumerate() {
            if i + d.len() <= s.len() && &s[i..i + d.len()] == d {
                first = n as u32;
                break 'outer;
            }
        }
    }

    let mut last: u32 = 11;
    // don't just copy paste .-.
    'outer: for i in (0..s.len()).rev() {
        if (s[i] as char).is_numeric() {
            last = (s[i] as char).to_digit(10).unwrap();
            break 'outer;
        }
        for (n, &d) in digits.iter().enumerate() {
            if i + d.len() <= s.len() && &s[i..i + d.len()] == d {
                last = n as u32;
                break 'outer;
            }
        }
    }

    // Make sure we actually got digits
    assert!(first < 10 && last < 10);

    first * 10 + last
}

fn main() {
    let input = include_str!("input.txt");
    let acc: u32 = input.lines().map(|l| str_to_digits(l.as_bytes())).sum();
    dbg!(acc);
}

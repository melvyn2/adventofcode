fn main() {
    let input: Vec<String> = include_str!("input")
        .split("\n\n")
        .map(|s| s.replace('\n', " ").trim().to_owned())
        .collect();

    let mut valid: u16 = 0;

    'pass: for doc in &input {
        for elem in ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter() {
            if !doc.contains(elem) {
                // dbg!(doc, elem);
                continue 'pass;
            }
        }
        for dp in doc.split(' ') {
            let dpv: Vec<&str> = dp.split(':').collect();
            // dbg!(&doc, &dpv);
            let elem = dpv[0];
            let data = dpv[1];
            match elem {
                "byr" => {
                    let byr: u16 = data.parse().unwrap();
                    if (1920 > byr) || (2002 < byr) {
                        dbg!(byr);
                        continue 'pass
                    }
                },
                "iyr" => {
                    let iyr: u16 = data.parse().unwrap();
                    if (2010 > iyr) || (2020 < iyr) {
                        dbg!(iyr);
                        continue 'pass
                    }
                },
                "eyr" => {
                    let eyr: u16 = data.parse().unwrap();
                    if (2020 > eyr) || (2030 < eyr) {
                        dbg!(eyr);
                        continue 'pass
                    }
                },
                "hgt" => {
                    match &data[(data.len()-2)..] {
                        "cm" => {
                            let hgt: u32 = data[..(data.len()-2)].parse().unwrap();
                            if (hgt < 150) || (hgt > 193) {
                                continue 'pass
                            }
                        },
                        "in" => {
                            let hgt: u32 = data[..(data.len()-2)].parse().unwrap();
                            if (hgt < 59) || (hgt > 76) {
                                continue 'pass
                            }
                        }
                        _ => continue 'pass
                    };
                },
                "hcl" => {
                    let bdata = data.as_bytes();
                    if (bdata[0] != b'#')
                        || (bdata.len() != 7)
                        || (i32::from_str_radix(&data[1..], 16).unwrap_or(0xffffff) >= 0xffffff)
                        { continue 'pass }
                },
                "ecl" => {
                    if ! ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&data) {
                        continue 'pass
                    }
                }
                "pid" => {
                    if (data.len() != 9) || (data.parse::<i32>().unwrap_or(-1) == -1) {
                        continue 'pass
                    }
                }
                _ => ()
            }
        }
        valid += 1;
    }                                                                                                                                                                                  
    println!("{}", valid)
}

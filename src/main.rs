fn main() {
    let mut seats_now: Vec<Vec<u8>> = include_str!("input")
        .lines()
        .map( |line| line.as_bytes().to_vec() )
        .collect();

    let mut seats_next = seats_now.clone();

    for y in 0..seats_now.len() {
        for x in 0..seats_now.len() {
            assert_eq!(seats_now[y][x], seats_next[y][x]);
        }
    }

    loop {
        for y in 0..seats_now.len() {
            'x: for (x, space) in seats_now[y].iter().enumerate() {
                // if x + y == 0 { unsafe { breakpoint(); } }
                match space {
                    b'.' => continue,
                    b'L' => {
                        for (xslope, yslope) in &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
                            for a in 1..100 {
                                let xoff = xslope * a;
                                let yoff = yslope * a;
                                if (y as isize + yoff) < 0 || (y as isize + yoff) >= seats_now.len() as isize { continue }
                                if (x as isize + xoff) < 0 || (x as isize + xoff) >= seats_now[y].len() as isize { continue }
                                if seats_now[(y as isize + yoff) as usize][(x as isize + xoff) as usize] == b'#' { continue 'x }
                                if seats_now[(y as isize + yoff) as usize][(x as isize + xoff) as usize] == b'L' { break }
                            }
                        }
                        seats_next[y][x] = b'#';
                    },
                    b'#' => {
                        let mut adj_occ: usize = 0;
                        for (xslope, yslope) in &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
                            for a in 1..100 {
                                let xoff = xslope * a;
                                let yoff = yslope * a;
                                if (y as isize + yoff) < 0 || (y as isize + yoff) >= seats_now.len() as isize { continue }
                                if (x as isize + xoff) < 0 || (x as isize + xoff) >= seats_now[y].len() as isize { continue }
                                if seats_now[(y as isize + yoff) as usize][(x as isize + xoff) as usize] == b'#' { adj_occ += 1; break }
                                if seats_now[(y as isize + yoff) as usize][(x as isize + xoff) as usize] == b'L' { break }
                            }
                        }
                        if adj_occ >= 5 {
                            seats_next[y][x] = b'L';
                        }
                    },
                    _ => panic!("invalid seat {} at {}, {}", *space as char, x, y)
                }
            }
            print!("{}      ", seats_now[y].iter().zip(seats_next[y].iter()).filter(|&(a, b)| a != b).count());
            println!("{}        {}", std::str::from_utf8(&*seats_now[y]).unwrap(), std::str::from_utf8(&*seats_next[y]).unwrap());
        }
        println!("\n");
        if seats_now == seats_next { break }
        seats_now = seats_next.clone();
        // sleep(Duration::from_secs(1));
    }

    let total_occupied: usize = seats_now.iter().fold(0, |acc, row | {
       row.iter().filter(|seat| **seat == b'#').count() + acc
    });
    dbg!(total_occupied);
}

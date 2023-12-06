fn main() {
    let input = include_str!("input.txt");

    let mut acc = 0;
    'game: for (idx, game) in input.lines().enumerate() {
        let draws = game.split_once(": ").unwrap().1.split("; ");
        for draw in draws {
            let (mut red, mut green, mut blue) = (0, 0, 0);
            for color_draw in draw.split(", ") {
                let (count, color) = color_draw.split_once(' ').unwrap();
                let n = count.parse::<u32>().unwrap();
                match color {
                    "red" => red += n,
                    "green" => green += n,
                    "blue" => blue += n,
                    _ => unreachable!(),
                }
            }
            if red > 12 || green > 13 || blue > 14 {
                continue 'game;
            }
        }
        acc += idx + 1;
    }
    dbg!(acc);
}

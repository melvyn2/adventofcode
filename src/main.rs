use std::cmp::max;

fn main() {
    let input = include_str!("input.txt");

    let mut acc = 0;
    for game in input.lines() {
        let mut min_colors = (0, 0, 0);
        let draws = game.split_once(": ").unwrap().1.split("; ");
        for draw in draws {
            let mut colors = (0, 0, 0);
            for color_draw in draw.split(", ") {
                let (count, color) = color_draw.split_once(' ').unwrap();
                let n = count.parse::<u32>().unwrap();
                match color {
                    "red" => colors.0 += n,
                    "green" => colors.1 += n,
                    "blue" => colors.2 += n,
                    _ => unreachable!(),
                }
            }
            min_colors.0 = max(min_colors.0, colors.0);
            min_colors.1 = max(min_colors.1, colors.1);
            min_colors.2 = max(min_colors.2, colors.2);
        }
        acc += min_colors.0 * min_colors.1 * min_colors.2;
    }
    dbg!(acc);
}

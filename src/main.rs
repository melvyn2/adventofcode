use std::collections::BTreeSet;

fn nudge_left_numeric(mut pos: usize, plane: &[u8], line_len: usize) -> usize {
    while (pos % line_len) > 0 && (plane[pos - 1] as char).is_numeric() {
        pos -= 1;
    }
    pos
}

fn expand_right_and_collect_numeric(pos: usize, plane: &[u8], line_len: usize) -> usize {
    let mut num = (plane[pos] as char).to_digit(10).unwrap();
    let mut len = 1;
    while ((pos % line_len) + len) <= line_len && (plane[pos + len] as char).is_numeric() {
        num = (num * 10) + (plane[pos + len] as char).to_digit(10).unwrap();
        len += 1;
    }
    num as usize
}

fn find_numeric(pos: usize, plane: &[u8], line_len: usize, set: &mut BTreeSet<usize>) -> bool {
    let mut acc = false;

    let x = pos % line_len;
    let y = pos / line_len;
    let xleft = x > 0;
    let xright = x < line_len - 1;
    if y > 0 {
        if xleft && (plane[pos - (line_len + 1)] as char).is_numeric() {
            acc |= set.insert(nudge_left_numeric(pos - (line_len + 1), plane, line_len));
        }
        if (plane[pos - line_len] as char).is_numeric() {
            acc |= set.insert(nudge_left_numeric(pos - (line_len), plane, line_len));
        }
        if xright && (plane[pos - (line_len - 1)] as char).is_numeric() {
            acc |= set.insert(nudge_left_numeric(pos - (line_len - 1), plane, line_len));
        }
    }
    if y < (plane.len() / line_len) - 1 {
        if xleft && (plane[pos + (line_len - 1)] as char).is_numeric() {
            acc |= set.insert(nudge_left_numeric(pos + (line_len - 1), plane, line_len));
        }
        if (plane[pos + line_len] as char).is_numeric() {
            acc |= set.insert(nudge_left_numeric(pos + (line_len), plane, line_len));
        }
        if xright && (plane[pos + (line_len + 1)] as char).is_numeric() {
            acc |= set.insert(nudge_left_numeric(pos + (line_len + 1), plane, line_len));
        }
    }
    if xleft && (plane[pos - 1] as char).is_numeric() {
        acc |= set.insert(nudge_left_numeric(pos - 1, plane, line_len));
    }
    if xright && (plane[pos + 1] as char).is_numeric() {
        acc |= set.insert(nudge_left_numeric(pos + 1, plane, line_len));
    }

    acc
}

fn main() {
    let input = include_bytes!("input.txt");

    let line_len = input
        .iter()
        .enumerate()
        .find(|(_, &b)| b == b'\n')
        .unwrap()
        .0;

    let plane = input
        .iter()
        .cloned()
        .filter(|&b| b != b'\n')
        .collect::<Vec<u8>>();

    let mut positions: BTreeSet<usize> = BTreeSet::new();
    let mut acc = 0;
    // Find all symbols, and collect all surrounding leftmost-number fragments
    for (pos, _) in plane.iter().enumerate().filter(|(_, &b)| b == b'*') {
        find_numeric(pos, &plane, line_len, &mut positions);
        if positions.len() == 2 {
            acc +=
                expand_right_and_collect_numeric(positions.pop_first().unwrap(), &plane, line_len)
                    * expand_right_and_collect_numeric(
                        positions.pop_first().unwrap(),
                        &plane,
                        line_len,
                    );
        } else {
            positions.clear()
        }
    }

    dbg!(acc);
}


fn main() {
    let input: Vec<String> = include_str!("input")
        .split("\n\n")
        .map( |s| s.replace('\n', "").to_string())
        .collect();

    let mut acc = 0;

    for group_raw in input {
        let mut group = group_raw.into_bytes();
        group.sort();
        group.dedup();
        dbg!(&group.len());
        acc += group.len();
    }
    dbg!(acc);
}

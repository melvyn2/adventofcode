use jemallocator::Jemalloc;
use pathfinding::prelude::dijkstra;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn successors(
    pos: &(i32, i32, char),
    map: &[Vec<char>],
    dbg_map: &mut Vec<Vec<char>>,
) -> std::vec::IntoIter<((i32, i32, char), i32)> {
    let mut res: Vec<((i32, i32, char), i32)> = vec![];
    for y_o in -1..=1 {
        for x_o in -1..=1 {
            let cand_pos = (pos.0 + y_o, pos.1 + x_o);
            if (cand_pos.0 - pos.0).abs() + (cand_pos.1 - pos.1).abs() == 1
                && map.get(cand_pos.0 as usize).is_some()
                && map[cand_pos.0 as usize].get(cand_pos.1 as usize).is_some()
            {
                let cand = match map[cand_pos.0 as usize][cand_pos.1 as usize] {
                    'E' => 'z',
                    'S' => 'a',
                    c => c,
                };
                if cand as i32 - pos.2 as i32 >= -1 {
                    res.push(((cand_pos.0, cand_pos.1, cand), 1));
                    dbg_map[cand_pos.0 as usize][cand_pos.1 as usize] = cand.to_ascii_uppercase();
                }
            }
        }
    }
    println!("At point: {:?}, cands: {:?}", &pos, &res);
    res.into_iter()
}

fn main() {
    let input = include_str!("input.txt");
    let map = input // Map of chars in Y+, X+
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut dbg_map = map.clone();

    let start = {
        let t = map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (y, x, c)))
            .find(|(_, _, &c)| c == 'S')
            .unwrap();
        (t.0 as i32, t.1 as i32, 'a')
    };
    let goal = {
        let t = map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (y, x, c)))
            .find(|(_, _, &c)| c == 'E')
            .unwrap();
        (t.0 as i32, t.1 as i32, 'z')
    };
    dbg!(&start, &goal);

    let a = dijkstra(
        &goal,
        |pos| successors(pos, &map, &mut dbg_map),
        |&pos| pos.2 == 'a',
    );

    for l in dbg_map {
        println!("{}", l.iter().collect::<String>());
    }

    dbg!(&a);
    dbg!(&a.map(|a| a.0.len() - 1));
}

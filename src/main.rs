#![feature(iter_advance_by)]

use std::collections::HashMap;
use std::path::PathBuf;

fn main() {
    let mut cmds = include_str!("input.txt").split("\n$ ");
    cmds.advance_by(1).unwrap();
    let mut pwd = PathBuf::from("/");
    let mut fs: HashMap<String, Vec<(&str, usize)>> = HashMap::new();
    for cmd in cmds {
        let (cmd_base, rest) = cmd.split_at(3);
        let mut lines = rest.lines();
        match cmd_base {
            "cd " => {
                let path = lines.next().unwrap();
                if path == ".." {
                    pwd.pop();
                } else {
                    pwd.push(path);
                }
                // dbg!(&pwd);
            }
            "ls\n" => {
                let contents = lines
                    .filter(|s| !s.starts_with("dir"))
                    .map(|l| {
                        let (size_s, name) = l.split_once(" ").unwrap();
                        (name, size_s.parse::<usize>().unwrap())
                    })
                    .collect::<Vec<(&str, usize)>>();
                fs.insert(pwd.to_string_lossy().to_string(), contents);
                // dbg!(&fs[pwd.to_str().unwrap()]);
            }
            _ => unreachable!(),
        }
    }

    let sum = fs
        .keys()
        .clone()
        .map(|dir| {
            fs.iter()
                .filter(|(d, _)| d.starts_with(dir))
                .map(|(_, files)| files.iter().map(|(_, s)| s).sum::<usize>())
                .sum::<usize>()
        })
        .filter(|&s| s <= 100000)
        .sum::<usize>();

    dbg!(sum);
}

use std::collections::{HashMap, HashSet};

type Bag = (String, String);

fn recurse_for_gold(rules: &HashMap<Bag, Vec<(usize, Bag)>>, bag: &Bag, acc: &mut HashSet<Bag>) -> bool {
    let mut contains_gold: bool = false;

    for (_number, bag_in) in rules.get(bag).unwrap() {
        if *bag_in == ("shiny".to_string(), "gold".to_string()) || recurse_for_gold(rules, bag_in, acc) {
            acc.insert(bag.clone());
            contains_gold = true;
        }
    }
    contains_gold
}

fn main() {
    let rules: HashMap<Bag, Vec<(usize, Bag)>> = include_str!("input")
        .split('\n')
        .map(|rule| {
            let rules_s: Vec<&str> = rule.split_whitespace().collect();
            let container: Bag = (rules_s[0].to_string(), rules_s[1].to_string());

            let subs: Vec<(usize, Bag)> = if rule.contains("no other bags.") {
                vec![]
            } else {
                rule.split("contain")
                    .collect::<Vec<&str>>()
                    [1]
                    .replace('.', " ")
                    .split(", ")
                    .map(|bag| {
                        let bag_s: Vec<&str> = bag.trim().split(' ').collect();
                        (bag_s[0].parse::<usize>().unwrap(), (bag_s[1].to_string(), bag_s[2].to_string()))
                    })
                    .collect()
            };
            (container, subs)
        })
        .collect();

    let mut acc: HashSet<Bag> = HashSet::new();
    for bag in rules.keys() {
        recurse_for_gold(&rules, &bag, &mut acc);
    }
    dbg!(acc.len(), rules.len());

}

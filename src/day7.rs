use std::collections::{HashMap, HashSet};
use regex::Regex;

pub fn solve_floor() {
    let bags_map = parse_bag_map();

    let mut cur_bags: Vec<&str> = vec!["shiny gold"];
    let mut seen_bags: HashSet<&str> = HashSet::new();

    while !cur_bags.is_empty() {
        let bag = cur_bags.pop().unwrap();
        seen_bags.insert(bag);

        let new_bags:Vec<&str> = bags_map
            .iter()
            .filter(|(_, bag_list)| bag_list.iter().any(|(_, ingredient)| bag == *ingredient))
            .map(|(&key, _)| key)
            .collect();

        for new_bag in new_bags {
            cur_bags.push(new_bag);
        }
    }

    // Exclude the shiny gold bag.
    println!("{}", seen_bags.len() - 1);
}

pub fn solve_basement() {
    let bags_map = parse_bag_map();

    let mut cur_bags: Vec<&str> = vec!["shiny gold"];
    let mut bag_count = 0;

    while !cur_bags.is_empty() {
        let cur_bag = cur_bags.pop().unwrap();
        for (num, containing_bag) in bags_map.get(cur_bag).unwrap() {
            for _ in 0..*num {
                bag_count += 1;
                cur_bags.push(*containing_bag);
            }
        }
    }

    println!("{}", bag_count);
}

fn parse_bag_map() -> HashMap<&'static str, Vec<(i32, &'static str)>> {
    let bags_map: HashMap<&str, Vec<(i32, &str)>> = include_str!("../resources/7-1.txt")
        .lines()
        .map(|line| {
            let mut line_itr = line.trim_end_matches(".").split("contain");
            let key = line_itr.next().unwrap().trim();
            let ingredients = line_itr.next().unwrap().trim();
            assert_eq!(line_itr.count(), 0, "We parsed the line wrong!");
            let values: Vec<(i32, &str)> = ingredients.split(",").map(|value| parse_bag_ingredient(value)).collect();
            (parse_bag_recipe(key), values)
        })
        .collect::<HashMap<&str, Vec<(i32, &str)>>>();
    bags_map
}


fn parse_bag_recipe(value: &str) -> &str {
    value.trim().trim_end_matches("bags").trim()
}

fn parse_bag_ingredient(value: &str) -> (i32, &str) {
    lazy_static! {
        static ref BAG_REGEX: Regex = Regex::new("^([0-9])([a-z ]+)bags?$").unwrap();
    }

    if value.trim() == "no other bags" {
        return (0, "");
    }

    let matches = BAG_REGEX.captures(value.trim()).unwrap();
    let number = matches.get(1).unwrap().as_str().parse().unwrap();
    let bag_desc = matches.get(2).unwrap().as_str().trim();

    (number, bag_desc)
}


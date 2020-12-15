use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn solve_floor() {
    let starting_numbers: Vec<usize> = include_str!("../resources/15-1.txt")
        .trim()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();

    let spoken_numbers: HashMap<usize, usize> = starting_numbers
        .iter()
        .enumerate()
        .map(|(i, &num)| (num, i + 1))
        .collect();

    println!("{}", solve(*starting_numbers.last().unwrap(), spoken_numbers, 2020));
}

pub fn solve_basement() {}

fn solve(starting_number: usize, mut spoken_numbers: HashMap<usize, usize>, iterations: usize) -> usize {
    let mut current_num = starting_number;
    for turn in spoken_numbers.len() + 1..iterations + 1 {
        current_num = match spoken_numbers.entry(current_num) {
            Entry::Occupied(mut occupied) => {
                turn - occupied.insert(turn - 1) - 1
            }
            Entry::Vacant(vacant) => {
                vacant.insert(turn - 1);
                0
            }
        }
    }

    current_num
}

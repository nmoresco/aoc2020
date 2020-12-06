use std::collections::HashSet;

const LETTERS: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
    'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

pub fn solve_floor() {
    let data: usize = include_str!("../resources/6-1.txt")
        .split("\n\n")
        .map(|line| {
            let chars: HashSet<char> = line
                .chars()
                .filter(|&ch| ch != '\n')
                .collect();
            chars
        })
        .map(|line| line.len())
        .sum();

    println!("{}", data);
}

pub fn solve_basement() {
    let data: usize = include_str!("../resources/6-1.txt")
        .split("\n\n")
        .map(|group| group.split('\n').collect())
        .map(|group: Vec<&str>| {
            LETTERS
                .iter()
                .filter(|&ch| group.iter().all(|line| line.contains(*ch)))
                .collect()
        })
        .map(|group: Vec<&char>| group.len())
        .sum();

    println!("{}", data);
}


use std::fs;

pub fn solve_floor() {
    let contents = fs::read_to_string("./resources/1-1.txt")
        .expect("Something went wrong reading the file");

    let entries: Vec<i32> = contents.lines().map(|line| line.parse().unwrap()).collect();
    let result = find_pair_match(entries);

    println!("{}", result);
}

pub fn solve_basement() {
    let contents = fs::read_to_string("./resources/1-1.txt")
        .expect("Something went wrong reading the file");

    let entries: Vec<i32> = contents.lines().map(|line| line.parse().unwrap()).collect();
    let result = find_triple_match(entries);

    println!("{}", result);
}

fn find_pair_match(entries: Vec<i32>) -> i32 {
    for i in 0..entries.len() {
        for j in 0..entries.len() {
            match entries[i] + entries[j] {
                2020 => return entries[i] * entries[j],
                _ => continue
            }
        }
    }

    return -1;
}

fn find_triple_match(entries: Vec<i32>) -> i32 {
    for i in 0..entries.len() {
        for j in 0..entries.len() {
            for k in 0..entries.len() {
                match entries[i] + entries[j] + entries[k] {
                    2020 => return entries[i] * entries[j] * entries[k],
                    _ => continue
                }
            }
        }
    }

    return -1;
}

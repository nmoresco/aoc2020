use regex::Regex;

pub fn solve_floor() {
    let mut num_valid = 0;

    let data = include_str!("../resources/2-1.txt").lines();
    for line in data {
        let (min_occurs, max_occurs, rule, password) = parse_password_info(line);

        let count = password.matches(rule as char).count();
        if count >= min_occurs && count <= max_occurs {
            num_valid += 1;
        }
    }

    println!("{}", num_valid);
}

pub fn solve_basement() {
    let mut num_valid = 0;

    let data = include_str!("../resources/2-1.txt").lines();
    for line in data {
        let (first_index, second_index, rule, password) = parse_password_info(line);
        let password = password.as_bytes();

        if (password[first_index - 1] == rule && password[second_index - 1] != rule)
            || (password[first_index - 1] != rule && password[second_index - 1] == rule) {
            num_valid += 1;
        }
    }

    println!("{}", num_valid);
}

fn parse_password_info(line: &str) -> (usize, usize, u8, String) {
    let regex = Regex::new("^(\\d+)-(\\d+) ([a-zA-Z]): ([a-zA-Z]+$)").unwrap();

    let captures = regex.captures(line).unwrap();
    let first_index: usize = captures.get(1).unwrap().as_str().parse().unwrap();
    let second_index: usize = captures.get(2).unwrap().as_str().parse().unwrap();
    let rule = captures.get(3).unwrap().as_str().as_bytes()[0];
    let password = captures.get(4).unwrap().as_str().to_string();
    (first_index, second_index, rule, password)
}

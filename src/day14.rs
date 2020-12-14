use std::collections::HashMap;

enum Operation<'a> {
    MASK((u64, u64)),
    FloatingMask(&'a str),
    ASSIGN((u64, u64)),
}

pub fn solve_floor() {
    let data: Vec<Operation> = include_str!("../resources/14-1.txt")
        .lines()
        .map(|line| {
            let mut split = line.split("=");
            let operator = split.next().unwrap().trim();
            let arg = split.next().unwrap().trim();

            if operator == "mask" {
                Operation::MASK(parse_mask(arg))
            } else {
                Operation::ASSIGN(parse_store(operator, arg))
            }
        }).collect();

    let mut cur_mask: (u64, u64) = (0, 0);
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for line in data {
        match line {
            Operation::MASK(mask) => {
                cur_mask = mask;
            }
            Operation::ASSIGN((location, value)) => {
                memory.insert(location, value & cur_mask.0 | cur_mask.1);
            }
            Operation::FloatingMask(_) => panic!("Shouldn't have gotten this type yet!")
        }
    }

    println!("{}", memory.values().sum::<u64>());
}

fn parse_mask(mask: &str) -> (u64, u64) {
    let and_mask: String = mask.chars().map(|ch| {
        match ch {
            '0' => '0',
            '1' | 'X' => '1',
            _ => panic!("Invalid char for mask")
        }
    }).collect();

    let or_mask: String = mask.chars().map(|ch| {
        match ch {
            '0' | 'X' => '0',
            '1' => '1',
            _ => panic!("Invalid char for mask")
        }
    }).collect();

    let and_mask = u64::from_str_radix(&and_mask, 2).unwrap();
    let or_mask = u64::from_str_radix(&or_mask, 2).unwrap();


    (and_mask, or_mask)
}

fn parse_store(store: &str, arg: &str) -> (u64, u64) {
    let location = store.trim_start_matches("mem[").trim_end_matches("]").parse().unwrap();
    let arg = arg.parse().unwrap();
    (location, arg)
}

pub fn solve_basement() {
    let data: Vec<Operation> = include_str!("../resources/14-1.txt")
        .lines()
        .map(|line| {
            let mut split = line.split("=");
            let operator = split.next().unwrap().trim();
            let arg = split.next().unwrap().trim();

            if operator == "mask" {
                Operation::FloatingMask(arg)
            } else {
                Operation::ASSIGN(parse_store(operator, arg))
            }
        }).collect();

    let mut cur_mask: &str = "";
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for line in data {
        match line {
            Operation::FloatingMask(mask) => {
                cur_mask = mask;
            }
            Operation::ASSIGN((location, value)) => {
                floating_assign(&apply_mask(location, cur_mask), value, &mut memory);
            }
            Operation::MASK(_) => panic!("Shouldn't have gotten this type here!")
        }
    }

    println!("{}", memory.values().sum::<u64>());
}

fn apply_mask(location: u64, mask: &str) -> String {
    let binary_string = format!("{:b}", location);
    // This glorious macro pads the string to the left with 0's until the string is 36 chars long.
    format!("{:0>36}", binary_string)
        .chars()
        .enumerate()
        .map(|(idx, ch)| {
            match mask.chars().nth(idx).unwrap() {
                '0' => ch,
                '1' => '1',
                'X' => 'X',
                _ => panic!("Invalid char for mask")
            }
        }).collect()
}

fn floating_assign(location: &str, value: u64, memory: &mut HashMap<u64, u64>) {
    if !location.contains("X") {
        let location: u64 = u64::from_str_radix(location, 2).unwrap();
        memory.insert(location, value);
    } else {
        floating_assign(&location.replacen("X", "0", 1), value, memory);
        floating_assign(&location.replacen("X", "1", 1), value, memory);
    }
}

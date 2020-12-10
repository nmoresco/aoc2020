struct Node {
    children: Vec<Node>,
    value: i64
}

pub fn solve_floor() {
    let data = parse_data();

    let mut one_jolts = 0;
    let mut three_jolts = 0;
    for i in 1..data.len() {
        let difference = data[i] - data[i - 1];
        match difference {
            1 => one_jolts += 1,
            2 => {}
            3 => three_jolts += 1,
            0 => panic!("Duplicate adapters!"),
            _ => panic!("Can't use all the adapters!")
        }
    }

    println!("{}", one_jolts * three_jolts);
}

pub fn solve_basement() {
    let data = parse_data();

    let root = Node {
        children: find_valid_adapters(&data[1..].to_vec(), 0),
        value: 0
    };

    println!("DAG built, finding combinations...");
    println!("{}", find_combinations(root, *data.iter().max().unwrap()));
}

fn find_combinations(root: Node, max_value: i64) -> i64 {
    let mut stack = vec!();
    let mut count = 0;
    stack.push(root);
    while !stack.is_empty() {
        let cur_node = stack.pop().unwrap();
        if cur_node.value == max_value {
            count += 1;
        }
        for child in cur_node.children {
            stack.push(child);
        }
    }

    count
}

fn find_valid_adapters(data: &Vec<i64>, cur_node: i64) -> Vec<Node> {
    let mut possible_adapters = Vec::new();
    for adapter in data {
        let difference = adapter - cur_node;
        if difference <= 0 {
            continue;
        }
        else if difference <= 3 {
            possible_adapters.push(*adapter);
        }
        else {
            break;
        }
    }

    possible_adapters
        .iter()
        .map(|&x| Node {
            children: find_valid_adapters(&data, x),
            value: x
        })
        .collect::<Vec<Node>>()
}

fn parse_data() -> Vec<i64> {
    let mut data: Vec<i64> = include_str!("../resources/10-1.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();
    data.sort();

    data.insert(0, 0);
    data.insert(data.len(), data.get(data.len() - 1).unwrap() + 3);
    data
}

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

    // Since there happens to be no numbers that are exactly 2 apart, you only need to worry about
    // 1 apart and 3 apart. Turns out, groups of 1 apart have permutations that follow the
    // tribonacci sequence. So find those and multiply em together.
    let slices = consecutive_slices(data);
    let answer: i64 = slices.iter().map(|x| tribonacci(x.len())).product();

    println!("{}", answer);
}

fn consecutive_slices(data: Vec<i64>) -> Vec<Vec<i64>> {
    let mut slice_start = 0;
    let mut result = Vec::new();
    for i in 1..data.len() {
        if data[i] - data[i - 1] != 1 {
            result.push(data[slice_start..i].to_vec());
            slice_start = i;
        }
    }
    if data.len() > 0 {
        result.push(data[slice_start..].to_vec());
    }
    result
}

fn tribonacci(number: usize) -> i64 {
    match number {
        0 => 0,
        1 => 1,
        2 => 1,
        3 => 2,
        _ => tribonacci(number - 1) + tribonacci(number - 2) + tribonacci(number - 3)
    }
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

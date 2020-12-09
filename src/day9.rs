pub fn solve_floor() {
    let data: Vec<i64> = include_str!("../resources/9-1.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();


    let mut start = 0;
    let mut end = 25;
    let outlier = loop {
        let preamble = data[start..end].to_vec();
        let candidate = data[end];
        if !contains_multiple(preamble, candidate) {
            break candidate;
        }

        start += 1;
        end += 1;
    };

    println!("{}", outlier);
}

pub fn solve_basement() {
    let data: Vec<i64> = include_str!("../resources/9-1.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    let target: i64 = 50047984;

    for i in 0..data.len() {
        let mut cur_sum = data[i];
        for j in i + 1..data.len() {
            cur_sum += data[j];
            if cur_sum == target {
                let min = data[i..j].iter().min().unwrap();
                let max = data[i..j].iter().max().unwrap();
                println!("{}", min + max);
                return;
            }
            else if cur_sum > target {
                break;
            }
        }
    }
}

fn contains_multiple(vector: Vec<i64>, number: i64) -> bool {
    for i in 0..vector.len() {
        for j in 0..vector.len() {
            if (vector[i] + vector[j]) == number && vector[i] != vector[j] {
                return true;
            }
        }
    }

    false
}

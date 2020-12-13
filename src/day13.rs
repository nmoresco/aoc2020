pub fn solve_floor() {
    let mut data = include_str!("../resources/13-1.txt").lines();
    let target: i32 = data.next().unwrap().parse().unwrap();
    let (bus_id, mins_to_wait) = data.next().unwrap()
                     .split(',')
                     .filter(|&ch| ch != "x")
                     .map(|ch| ch.parse().unwrap())
                     .map(|num: f32| {
                         let nearest_departure:f32 = target as f32 / num;
                         (num as i32, (nearest_departure.ceil() * num) as i32 - target)
                     })
                     .min_by(|(_, prev_time_to_wait), (_, time_to_wait)| {
                         prev_time_to_wait.cmp(time_to_wait)
                     }).unwrap();
    println!("{}", bus_id * mins_to_wait);
}

// This solution only works on a 64-bit machine.
pub fn solve_basement() {
    let data: Vec<(usize, usize)> = include_str!("../resources/13-1.txt").lines()
        .last().unwrap()
        .split(',')
        .enumerate()
        .filter(|&(_idx, ch)| ch != "x")
        .map(|(idx, ch)| (idx, ch.parse().unwrap()))
        .collect();

    let mut cur_interval = data[0].1;
    let mut i = 1;
    let mut x = cur_interval;

    // Here's my best attempt at explaining this: by experimentation the numbers repeat the cycle
    // every x * y times. So if your numbers are 7 and 13, they will repeat every 7 * 13 times (after
    // the first time.)  If they are 7, 13, and 59, they will repeat every 7 * 13 * 59 times.
    // So only increment on those intervals til you find the next intersection. Repeat til done.
    loop {
        let (cur_position, cur_number) = data[i];
        if (x + cur_position) % cur_number == 0 {
            cur_interval *= cur_number;
            i += 1;
            if i >= data.len() {
                break;
            }
        }

        x += cur_interval
    }

    println!("{}", x)
}


use crate::util::{create_board, print_board};

pub fn solve_floor() {
    let mut data = create_board(include_str!("../resources/11-1.txt"));

    let mut prev_data: Vec<Vec<char>> = vec![];

    while !data.eq(&prev_data) {
        let new_data = simulate_seats(&data);
        prev_data = data;
        data = new_data;
    }

    print_board(&data);

    println!("{}", data.iter().flatten().filter(|&&x| x == '#').count());
}

pub fn solve_basement() {
    let mut data = create_board(include_str!("../resources/11-1.txt"));

    print_board(&data);
    println!("==========");

    let mut prev_data: Vec<Vec<char>> = vec![];

    while !data.eq(&prev_data) {
        let new_data = simulate_seats2(&data);
        prev_data = data;
        data = new_data;
        print_board(&data);
        println!("==========");
    }

    print_board(&data);

    println!("{}", data.iter().flatten().filter(|&&x| x == '#').count());
}

fn simulate_seats(data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut next_step = data.clone();
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            match data[i][j] {
                '.' => {}
                'L' => {
                    if get_num_adjacent(data, i, j) == 0 {
                        next_step[i][j] = '#'
                    }
                }
                '#' => {
                    if get_num_adjacent(data, i, j) >= 4 {
                        next_step[i][j] = 'L'
                    }
                }
                _ => panic!("Found an unknown seat type"),
            }
        }
    }

    next_step
}

fn get_num_adjacent(data: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let directions: Vec<(isize, isize)> = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let width = data.len() as isize;
    let height = data[0].len() as isize;

    directions
        .iter()
        // map directions to true coordinates
        .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
        // make sure the coordinates didn't go out of bounds
        .filter(|(fx, fy)| {
            *fx >= 0
                && *fx < width
                && *fy >= 0
                && *fy < height
        })
        // map those into real array positions
        .map(|(fx, fy)| data[fx as usize][fy as usize])
        .filter(|&ch| ch == '#')
        .count()
}

fn simulate_seats2(data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut next_step = data.clone();
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            match data[i][j] {
                '.' => {}
                'L' => {
                    if get_num_visible(data, i, j) == 0 {
                        next_step[i][j] = '#'
                    }
                }
                '#' => {
                    if get_num_visible(data, i, j) >= 5 {
                        next_step[i][j] = 'L'
                    }
                }
                _ => panic!("Found an unknown seat type"),
            }
        }
    }

    next_step
}

fn get_num_visible(data: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut found_seats: usize = 0;

    let directions: Vec<(isize, isize)> = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let width = data.len() as isize;
    let height = data[0].len() as isize;

    for (dx, dy) in directions {
        let mut fx: isize = dx + x as isize;
        let mut fy: isize = dy + y as isize;
        while fx >= 0
            && fx < width
            && fy >= 0
            && fy < height {
            match data[fx as usize][fy as usize] {
                '#' => {
                    found_seats += 1;
                    break;
                }
                'L' => {
                    break;
                }
                _ => {}
            }

            fx += dx;
            fy += dy;
        }
    }

    found_seats
}

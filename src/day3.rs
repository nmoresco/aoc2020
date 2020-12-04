use crate::util::{create_board};

pub fn solve_floor() {
    let data = include_str!("../resources/3-1.txt");
    let array = create_board(data);

    println!("{}", traverse_slope(&array, 3, 1));
}

pub fn solve_basement() {
    let data = include_str!("../resources/3-1.txt");
    let array = create_board(data);

    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let num_trees: u64 = slopes
        .iter()
        .map(|(x, y)| traverse_slope(&array, *x, *y))
        .product();

    println!("{}", num_trees);
}

fn traverse_slope(array: &[Vec<char>], x_steps: usize, y_steps: usize) -> u64 {
    let mut count = 0;
    let mut x: usize = 0;
    let mut y: usize = 0;
    let width = array[0].len();
    let height = array.len();

    while y < height {
        if array[y][x % width] == '#' {
            count += 1;
        }
        x += x_steps;
        y += y_steps;
    }

    count
}

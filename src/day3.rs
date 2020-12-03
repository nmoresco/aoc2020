use crate::util::create_board_char;

pub fn solve_floor() {
    let data = include_str!("../resources/3-1.txt");
    let array = create_board_char(data);

    println!("{}", traverse_slope(&array, 3, 1));
}

pub fn solve_basement() {
    let data = include_str!("../resources/3-1.txt");
    let array = create_board_char(data);

    let mut num_trees: u64 = traverse_slope(&array, 1, 1);
    num_trees *= traverse_slope(&array, 3, 1);
    num_trees *= traverse_slope(&array, 5, 1);
    num_trees *= traverse_slope(&array, 7, 1);
    num_trees *= traverse_slope(&array, 1, 2);

    println!("{}", num_trees);
}

fn traverse_slope(array: &Vec<Vec<char>>, x_steps: usize, y_steps: usize) -> u64 {
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

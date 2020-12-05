use std::cmp::{min};
use std::collections::HashMap;

pub fn solve_floor() {
    let data = include_str!("../resources/5-1.txt");

    let (seat_rows, seat_cols) = get_seats(data);

    let max_id = seat_rows
        .iter()
        .zip(seat_cols)
        .map(|(row, col)| row * 8 + col)
        .max()
        .unwrap();

    println!("{}", max_id);
}

pub fn solve_basement() {
    let data = include_str!("../resources/5-1.txt");
    let (seat_rows, seat_cols) = get_seats(data);

    // Convert our rows to a map, where the key is the row and the value is the number of seats in it.
    let mut row_map: HashMap<i32, i32> = HashMap::new();
    for row in seat_rows.iter() {
        if row_map.contains_key(row) {
            row_map.insert(*row, row_map.get(row).unwrap() + 1);
        }
        else {
            row_map.insert(*row, 1);
        }
    }

    // Filter out the rows that don't have 8 seats.
    let my_row :i32 = row_map
        .iter()
        .filter(|&(_key, value)| *value != 8)
        .map(|(key, _value)| *key)
        // This doesn't work generally - I happen to know that none of the bottom rows are partially missing.
        .min()
        .unwrap();

    // For every col in our row, sum them and find the missing number.
    let col_sum = seat_rows
        .iter()
        .zip(seat_cols)
        .filter(|&(row, _col)| *row == my_row)
        .fold(0, |acc, (_row, col)| acc + col);

    // 28 is 0 + 1 + 2 + 3 .. + 7
    let my_col = 28 - col_sum;

    println!("{}", my_row * 8 + my_col);
}

fn get_seats(data: &str) -> (Vec<i32>, Vec<i32>) {
    let seat_rows: Vec<i32> = data
        .trim()
        .split('\n')
        .map(|line| line[..7].to_string())
        .map(|line| {
            let mut seat_start = 0;
            let mut seat_end = 127;
            for char in line.chars() {
                if char == 'F' {
                    seat_end = (seat_end + seat_start) / 2;
                } else if char == 'B' {
                    seat_start = (seat_end + seat_start) / 2 + 1;
                } else {
                    panic!("Invalid char detected! Expected F or B")
                }
            }
            min(seat_start, seat_end)
        }).collect();

    let seat_cols: Vec<i32> = data
        .trim()
        .split('\n')
        .map(|line| line[7..].to_string())
        .map(|line| {
            let mut seat_start = 0;
            let mut seat_end = 8;
            for char in line.chars() {
                if char == 'L' {
                    seat_end = (seat_end + seat_start) / 2 + 1;
                } else if char == 'R' {
                    seat_start = (seat_end + seat_start) / 2;
                } else {
                    panic!("Invalid char detected! Expected R or L")
                }
            }
            min(seat_start, seat_end)
        }).collect();

    (seat_rows, seat_cols)
}


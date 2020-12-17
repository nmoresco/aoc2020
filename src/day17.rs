use std::collections::HashMap;

pub fn solve_floor() {
    let mut board: HashMap<(isize, isize, isize, isize), char> = include_str!("../resources/17-1.txt")
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, char)| ((row as isize, col as isize, 0 as isize, 0 as isize), char))
        })
        .flatten()
        .collect();

    let start_dimension = include_str!("../resources/17-1.txt").lines().count() as isize;
    let mut active_start_dimensions: (isize, isize, isize, isize) = (0, 0, 0, 0);
    let mut active_end_dimensions = (start_dimension, start_dimension, 0, 0);

    for _i in 0..6 {
        // Expand active dimensions
        active_start_dimensions = (active_start_dimensions.0 - 1,
                                   active_start_dimensions.1 - 1,
                                   active_start_dimensions.2 - 1,
                                   active_start_dimensions.3 - 1);
        active_end_dimensions = (active_end_dimensions.0 + 1,
                                 active_end_dimensions.1 + 1,
                                 active_end_dimensions.2 + 1,
                                 active_end_dimensions.3 + 1);

        let mut active_board = board.clone();

        for x in active_start_dimensions.0..active_end_dimensions.0 + 1 {
            for y in active_start_dimensions.1..active_end_dimensions.1 + 1 {
                for z in active_start_dimensions.2..active_end_dimensions.2 + 1 {
                    for w in active_start_dimensions.3..active_end_dimensions.3 + 1 {
                        active_board.insert((x, y, z, w), step(x, y, z, w, *active_board.get(&(x, y, z, w)).unwrap_or(&'.'), &board));
                    }
                }
            }
        }

        board = active_board;

        print_board(&board, active_start_dimensions, active_end_dimensions);
    }

    println!("{}", board.values().filter(|&&char| char == '#').count());
}

pub fn solve_basement() {}

fn step(x: isize, y: isize, z: isize, w: isize, char: char, board: &HashMap<(isize, isize, isize, isize), char>) -> char {
    let mut count_active = 0;
    for dx in (x - 1)..(x + 2) {
        for dy in (y - 1)..(y + 2) {
            for dz in (z - 1)..(z + 2) {
                for dw in (w - 1)..(w + 2) {
                    if (x, y, z, w) == (dx, dy, dz, dw) {
                        continue;
                    }
                    count_active += match board.get(&(dx, dy, dz, dw)) {
                        Some(&occ) => { if occ == '#' { 1 } else { 0 } }
                        None => { 0 }
                    }
                }
            }
        }
    }

    match char {
        '#' => {
            return if count_active == 2 || count_active == 3 {
                '#'
            } else {
                '.'
            };
        }
        '.' => {
            return if count_active == 3 {
                '#'
            } else {
                '.'
            };
        }
        _ => panic!("Weird char found in board!")
    }
}

fn print_board(board: &HashMap<(isize, isize, isize, isize), char>,
               start_dimensions: (isize, isize, isize, isize),
               end_dimensions: (isize, isize, isize, isize)) {
    println!("=================");
    for w in start_dimensions.3..end_dimensions.3 + 1 {
        for z in start_dimensions.2..end_dimensions.2 + 1 {
            println!("z={}, w={}", z, w);
            for x in start_dimensions.0..end_dimensions.0 + 1 {
                for y in start_dimensions.1..end_dimensions.1 + 1 {
                    print!("{}", board.get(&(x, y, z, w)).unwrap())
                }
                println!();
            }

            println!();
        }
    }
    println!("=================");
}

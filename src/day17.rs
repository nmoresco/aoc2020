use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn solve_floor() {
    let mut board: HashMap<(isize, isize, isize), char> = include_str!("../resources/17-1.txt")
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, char)| ((row as isize, col as isize, 0 as isize), char))
        })
        .flatten()
        .collect();

    for _i in 0..6 {
        board.clone().iter().for_each(|((x, y, z), &char)| {
            step(*x, *y, *z, char, &mut board);
        });

        print_board(&board);
    }

    println!("{}", board.values().filter(|&&char| char == '#').count());
}

pub fn solve_basement() {}

fn step(x: isize, y: isize, z: isize, char: char, board: &mut HashMap<(isize, isize, isize), char>) {
    let mut count_active = 0;
    for dx in (x - 1)..(x + 2) {
        for dy in (y - 1)..(y + 2) {
            for dz in (z - 1)..(z + 2) {
                count_active += match board.entry((dx, dy, dz)) {
                    Entry::Occupied(occ) => {
                        if occ.get() == &'#' { 1 } else { 0 }
                    }
                    Entry::Vacant(vac) => {
                        vac.insert('.');
                        0
                    }
                }
            }
        }
    }

    match char {
        '#' => {
            if count_active != 2 && count_active != 3 {
                board.insert((x, y, z), '.');
            }
        }
        '.' => {
            if count_active == 3 {
                board.insert((x, y, z), '#');
            };
        }
        _ => panic!("Weird char found in board!")
    }
}

fn print_board(board: &HashMap<(isize, isize, isize), char>) {
    let max_x: isize = board.iter().map(|(&(x, _, _), _)| x).max().unwrap();
    let min_x: isize = board.iter().map(|(&(x, _, _), _)| x).min().unwrap();

    let max_y: isize = board.iter().map(|(&(_, y, _), _)| y).max().unwrap();
    let min_y: isize = board.iter().map(|(&(_, y, _), _)| y).min().unwrap();

    let max_z: isize = board.iter().map(|(&(_, _, z), _)| z).max().unwrap();
    let min_z: isize = board.iter().map(|(&(_, _, z), _)| z).min().unwrap();

    println!("=================");
    for z in min_z..max_z + 1 {
        println!("z={}", z);
        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                print!("{}", board.get(&(x, y, z)).unwrap())
            }
            println!();
        }

        println!();
    }
    println!("=================");
}

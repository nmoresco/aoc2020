// #[macro_use]
// extern crate lazy_static;

use std::env;

mod util;
mod day10;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1].to_lowercase() != "basement" {
        day10::solve_floor();
    }
    else {
        day10::solve_basement();
    }
}

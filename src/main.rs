// #[macro_use]
// extern crate lazy_static;

use std::env;

mod util;
mod day15;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1].to_lowercase() != "basement" {
        day15::solve_floor();
    }
    else {
        day15::solve_basement();
    }
}

// #[macro_use]
// extern crate lazy_static;

use std::env;

mod util;
mod day9;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1].to_lowercase() != "basement" {
        day9::solve_floor();
    }
    else {
        day9::solve_basement();
    }
}

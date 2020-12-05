#[macro_use]
extern crate lazy_static;

use std::env;

mod util;
mod day5;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1].to_lowercase() != "basement" {
        day5::solve_floor();
    }
    else {
        day5::solve_basement();
    }
}

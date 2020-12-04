#[macro_use]
extern crate lazy_static;

use std::env;

mod day4;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1].to_lowercase() != "basement" {
        day4::solve_floor();
    }
    else {
        day4::solve_basement();
    }
}

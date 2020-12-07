#[macro_use]
extern crate lazy_static;

use std::env;

mod util;
mod day7;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1].to_lowercase() != "basement" {
        day7::solve_floor();
    }
    else {
        day7::solve_basement();
    }
}

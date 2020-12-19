// #[macro_use]
// extern crate lazy_static;

use std::env;

mod util;
mod day18p1;
mod day18_util;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1].to_lowercase() != "basement" {
        day18p1::solve_floor();
    }
    else {
        day18p1::solve_basement();
    }
}

use std::env;

mod day3;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1].to_lowercase() != "basement" {
        day3::solve_floor();
    }
    else {
        day3::solve_basement();
    }
}

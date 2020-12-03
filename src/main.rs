use std::env;

mod day1;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1].to_lowercase() != "basement" {
        day1::solve_floor();
    }
    else {
        day1::solve_basement();
    }
}

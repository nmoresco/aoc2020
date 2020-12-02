use std::env;

mod day2;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1].to_lowercase() != "basement" {
        day2::solve_floor();
    }
    else {
        day2::solve_basement();
    }
}

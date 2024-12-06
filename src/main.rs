use std::env;

#[path = "./day6/day6.rs"] mod day;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    day::solve();

    println!("Hello, world!");
}
use std::env;

#[path = "./day3/day3.rs"] mod day;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    day::solve();

    println!("Hello, world!");
}
use std::env;

#[path = "./day2/day2.rs"] mod day;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    day::solve();

    println!("Hello, world!");
}
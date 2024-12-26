#![feature(linked_list_cursors)]
use std::env;
use std::time::Instant;

#[path = "./day19/day19.rs"] mod day;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let now = Instant::now();

    day::solve();

    let elapsed = now.elapsed();

    println!("Elapsed: {:?}", elapsed);
}
use std::fs::read_to_string;
use regex::Regex;

pub fn solve() {
    let file_name: &str = "src/day3/input.txt";

    let mut result = 0;

    let rx = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();

    let mut proc = true;

    for line in read_to_string(file_name).unwrap().lines() {
        for cap in rx.captures_iter(line) {
            let s = cap.get(0).unwrap().as_str();

            if s == "do()" {
                proc = true;
                continue;
            }

            if s == "don't()" {
                proc = false;
                continue;
            }

            if proc == false {
                continue;
            }

            let fst = &s[4..s.len()-1].split(",").collect::<Vec<&str>>();

            result += fst[0].parse::<i32>().unwrap() * fst[1].parse::<i32>().unwrap();
        }
    }

    println!("Result: {}", result);
}
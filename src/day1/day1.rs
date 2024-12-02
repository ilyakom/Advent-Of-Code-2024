use std::{collections::{HashMap, HashSet}, fs::read_to_string};

pub fn solve() {
    println!("Day 1");

    let mut left = HashSet::new();
    let mut right = HashMap::new();

    let file_name: &str = "src/day1/input.txt";

    for line in read_to_string(file_name).unwrap().lines() {
        let parts = line.split("   ").collect::<Vec<&str>>();
        left.insert(parts[0].parse::<i32>().unwrap());

        let k = parts[1].parse::<i32>().unwrap();

        if right.contains_key(&k) {
            right.insert(k, right.get(&k).unwrap() + 1);
        } else {
            right.insert(k, 1);
        }
    }

    let mut result = 0;

    for i in &left {
        if right.contains_key(&i) {
            result += (i * right[i]).abs();
        }
    }

    println!("Result: {}", result);
}
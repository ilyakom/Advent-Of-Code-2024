use std::fs::read_to_string;

pub fn solve() {
    let file_name: &str = "src/day2/input.txt";

    let mut result = 0;

    for line in read_to_string(file_name).unwrap().lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();

        if check_row(parts, true) {
            result += 1;
        }
    }

    println!("Result: {}", result);
}

fn check_row (parts:Vec<&str>, skip: bool) -> bool {
    let inc:bool = parts[0].parse::<i32>().unwrap() < parts[1].parse::<i32>().unwrap();

    for i in 1..parts.len() {

        let first = parts[i-1].parse::<i32>().unwrap();
        let second = parts[i].parse::<i32>().unwrap();

        if inc && (first > second || (second - first).abs() > 3) {
            return if skip {try_skip(parts)} else {false};
        }
        
        if !inc && (first < second || (first - second).abs() > 3) {
            return if skip {try_skip(parts)} else {false};
        }
        
        if first == second {
            return if skip {try_skip(parts)} else {false};
        }
    }

    return true;
}

fn try_skip (parts:Vec<&str>) -> bool {

    for i in 0..parts.len() {
        let parts_ref = [&parts[..i], &parts[i+1..]].concat();

        let result = check_row(parts_ref, false);

        if result {
            return true;
        }
    }

    return false;
}
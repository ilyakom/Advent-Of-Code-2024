use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub fn solve() {
    let file_name: &str = "src/day5/input.txt";

    let mut result = 0;

    let mut map = HashMap::new();
    let lines = read_to_string(file_name).unwrap();

    let robot_x = 0;
    let robot_y = 0;

    for line in lines.lines() {
        if line.len() == 0 {
            rules = false;
            continue;
        }

        if rules {
            let parts = line.split("|").collect::<Vec<&str>>();
            let key = parts[0].parse::<i32>().unwrap();
            let value = parts[1].parse::<i32>().unwrap();

            if map.contains_key(&key) {
                let set: &mut HashSet<i32> = map.get_mut(&key).unwrap();
                set.insert(value);
            }
            else {
                let mut hs = HashSet::new();
                hs.insert(value);
                map.insert(key, hs);
            }
            
        } else {
            let parts = line.split(",").collect::<Vec<&str>>();
            let mut items = HashMap::new();

            for i in 0..parts.len() {
                let i:i32 = i as i32;
                items.insert(parts[i as usize].parse::<i32>().unwrap(), i);
            }

            if verify_rules(&map, &mut items) {
                let desired = (parts.len() as i32- 1) / 2;
                result += items.iter().find_map(|(key, &val)| if val == desired { Some(key) } else { None }).unwrap();
            }
        }

        
    }

    println!("Result: {}", result);
}

fn verify_rules(map: &HashMap<i32, HashSet<i32>>, items: &mut HashMap<i32, i32>) -> bool {

    let mut is_bad = false;
    let mut is_error = true;
    
    while is_error {
        is_error = false;

        for (key, value) in map {
            if items.contains_key(&key) {
                for v in value {
                    if items.contains_key(v) {
                        let i = items[key];
                        let j = items[v];

                        if i > j {
                            items.insert(*key, j);
                            items.insert(*v, i);

                            is_bad = true;
                            is_error = true;
                        }
                    }
                }
            }
        }
    }

    return is_bad;
}
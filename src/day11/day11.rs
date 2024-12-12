#![allow(unstable_features)]

use std::fs::read_to_string;
use std::collections::{HashMap, LinkedList};

pub fn solve() {
    let file_name: &str = "src/day11/input.txt";

    let lines = read_to_string(file_name).unwrap();
    let mut nums: Vec<u64> = Vec::new();

    for line in lines.lines() {
        nums = line.split(" ").map(|x| { x.parse::<u64>().unwrap() }).collect::<Vec<u64>>();

        println!("{:?}", nums);
    }

    let mut mem = nums.iter().map(|x| { (*x, 1) }).collect::<HashMap<u64, u64>>();

    println!("{:?}", mem);

    let steps = 1;

    for _k in 1..76 {
        let mut new_mem = HashMap::new();

        for num in mem.keys() {
            let new_nums = solve_for(*num, steps);
            
            for nn in new_nums {
                if new_mem.contains_key(&nn) {
                    new_mem.insert(nn, new_mem[&nn] + mem[num]);
                } else {
                    new_mem.insert(nn, mem[num]);
                }
            }
        }
    
        mem = new_mem;
    
        println!("Result {}: {}", _k * steps, mem.values().sum::<u64>());
    }
}

fn solve_for(num:u64, steps:usize) -> Vec<u64> {

    let mut nums: Vec<u64> = vec![num];

    for _i in 0..steps {
        let mut new_nums = Vec::new();
       
        for i in 0..nums.len() {
            if nums[i] == 0 {
                nums[i] = 1;
            } else if nums[i].to_string().len() % 2 == 0 {
                let s = nums[i].to_string();
                let fst = s.chars().collect::<Vec<char>>()[0..s.len() / 2].iter().collect::<String>().parse::<u64>().unwrap();
                let scnd = s.chars().collect::<Vec<char>>()[s.len() / 2..].iter().collect::<String>().parse::<u64>().unwrap();
                
                nums[i] = fst;
                new_nums.push(scnd);
            } else  {
                nums[i] = nums[i] * 2024;
            }
        }

        for a in new_nums {
            nums.push(a);
        }
    }

    return nums; 
}

fn part_1() {
    let file_name: &str = "src/day11/input.txt";

    let lines = read_to_string(file_name).unwrap();
    let mut nums: LinkedList<u64> = LinkedList::new();

    for line in lines.lines() {
        nums = line.split(" ").map(|x| { x.parse::<u64>().unwrap() }).collect::<LinkedList<u64>>();

        println!("{:?}", nums);
    }

    for _i in 0..25 {
        let mut cursor = nums.cursor_front_mut();

        while let Some(current) = cursor.current() {
            if *current == 0 {
                *current = 1;
            } else if current.to_string().len() % 2 == 0 {
                let s = current.to_string();
                let f = s.chars().collect::<Vec<char>>()[0..s.len() / 2].iter().collect::<String>().parse::<u64>().unwrap();
                let s = s.chars().collect::<Vec<char>>()[s.len() / 2..].iter().collect::<String>().parse::<u64>().unwrap();
                *current = s;
                cursor.insert_before(f);
            } else  {
    
                *current = *current * 2024;
            }
    
            cursor.move_next();
        }

        //println!("{:?}", nums);
    }
    
    //let mut result = 0;

    println!("Result: {}", nums.len());

}
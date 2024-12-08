use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};

pub fn solve() {
    let file_name: &str = "src/day8/input.txt";


    let mut antenas: HashMap<char, Vec<(i32,i32)>> = HashMap::new();

    let lines = read_to_string(file_name).unwrap();

    let mut j = 0;
    let mut i = 0;

    let mut result: HashSet<(i32, i32)> = HashSet::new();

    for line in lines.lines() {
        let chars = line.chars().collect::<Vec<char>>();

        i = 0;
        for ch in chars {
            if ch != '.' {
                if antenas.contains_key(&ch) {
                   antenas.get_mut(&ch).unwrap().push((j, i));
                } else {
                    antenas.insert(ch, vec![(j, i)]);
                }
            }
            i += 1;
        }

        j += 1;
    }

    for ant in &antenas {
        count_anti(&ant.1, &mut result, j, i);
    }

    println!("Result: {}", result.len());

}

fn count_anti(col: &Vec<(i32, i32)>, result: &mut HashSet<(i32, i32)>, x_max:i32, y_max:i32) {
    for i in 0..col.len() {
        let x_1 = col[i].0;
        let y_1 = col[i].1;

        for j in i+1..col.len() {
            let x_2 = col[j].0;
            let y_2 = col[j].1;

            let mut x_1_new:i32;
            let mut x_2_new:i32;

            let mut y_1_new:i32;
            let mut y_2_new:i32;

            let x_delta = (x_2 - x_1).abs();
            let y_delta = (y_2 - y_1).abs();

            let mut step = 0;

            loop {
                if x_1 < x_2 {
                    x_1_new = x_1 - x_delta * step;
                } else {
                    x_1_new = x_1 + x_delta * step;
                }

                if y_1 < y_2 {
                    y_1_new = y_1 - y_delta * step;
                } else {
                    y_1_new = y_1 + y_delta * step;
                }

                if x_1_new >= 0 && x_1_new < x_max && y_1_new >= 0 && y_1_new < y_max {
                    result.insert((x_1_new, y_1_new));
                } else {
                    break;
                }

                step += 1;
            }

            step = 0;

            loop {
                if x_1 < x_2 {
                    x_2_new = x_2 + x_delta * step;
                } else {
                    x_2_new = x_2 - x_delta * step;
                }

                if y_1 < y_2 {
                    y_2_new = y_2 + y_delta * step;
                } else {
                    y_2_new = y_2 - y_delta * step;
                }

                if x_2_new >= 0 && x_2_new < x_max && y_2_new >= 0 && y_2_new < y_max {
                    result.insert((x_2_new, y_2_new));
                } else {
                    break;
                }

                step += 1;
            }
        }
    }
}
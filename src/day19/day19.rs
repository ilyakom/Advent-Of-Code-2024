use std::{collections::{HashMap, HashSet, VecDeque}, fs::read_to_string};

pub fn solve() {
    let file_name: &str = "src/day19/input.txt";

    let lines = read_to_string(file_name).unwrap();
    let mut all_towels: Vec<&str> = Vec::new();
    let mut all_patterns: Vec<&str> = Vec::new();

    let mut reading_towels = true;

    for line in lines.lines() {
        if line == "" {
            reading_towels = false;
            continue;
        }

        if reading_towels {
            all_towels = line.split(", ").collect();
        } else {
            all_patterns.push(line);
        }
    }

    let mut result_2: usize = 0;
    let mut result_1 = 0;

    for pat in all_patterns {
        let mut hs: HashMap<usize, Vec<usize>> = HashMap::new();

        for i in 0..all_towels.len() {
            let tow = all_towels[i];
            let fnd_col:Vec<_> = get_idxs(pat, tow);

            for fnd in fnd_col {
                if hs.contains_key(&fnd) {
                    hs.get_mut(&fnd).unwrap().push(fnd + tow.len());
                } else {
                    let new_vec = vec![fnd + tow.len()];
                    hs.insert(fnd, new_vec);
                }
            }            
        }

        let mut mem = HashMap::new();
        let pat_res = search(&hs, 0, pat.len(), &mut mem);
        result_2 += pat_res;

        if pat_res > 0 {
            result_1 += 1;
        }

        println!("{} - {}", pat, pat_res);
    }

    println!("Result 1: {}. Ressult 2: {}", result_1, result_2);
}

fn get_idxs(sng:&str, pat:&str) -> Vec<usize> {
    let mut idxs: Vec<usize> = Vec::new();
    let mut idx = 0;

    while idx < sng.len() {
        let found = sng[idx..].starts_with(&pat);

        if found {
            idxs.push(idx);
        }

        idx += 1;
    }

    return idxs;
}

fn search(hs: &HashMap<usize, Vec<usize>>, idx:usize, len:usize, mem: &mut HashMap<usize, usize>) -> usize {
    if idx >= len {
        return 1;
    }

    if mem.contains_key(&idx) {
        return *(mem.get(&idx).unwrap());
    }

    if !hs.contains_key(&idx) {
        return 0;
    }

    let found = hs.get(&idx).unwrap();

    let mut result: usize = 0;

    for i in 0..found.len() {
        let f = found[i];
        let num = search(hs, f, len, mem);
        result += num;
    }

    mem.insert(idx, result);

    return result;
}
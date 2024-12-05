use std::fs::read_to_string;

pub fn solve() {
    let file_name: &str = "src/day4/input.txt";

    let mut result_1 = 0;
    let mut result_2 = 0;

    let mut map = Vec::new();
    let lines = read_to_string(file_name).unwrap();

    for line in lines.lines() {
        map.push(line);
    }

    for i in 0..map.len() {
        let i:i32 = i as i32;
        for j in 0..map[i as usize].len() {
            let j:i32 = j as i32;

            result_1 += find_1(&map, i, j, 'X');

            if find_2(&map, i, j) {
                result_2 += 1;
            }
        }
    }

    println!("Result 1: {}, Result 2: {}", result_1, result_2);
}

fn find_2(map: &Vec<&str>, i:i32, j:i32) -> bool {
    let cur = map[i as usize].chars().nth(j as usize).unwrap();

    if cur == 'A' {
        if i-1 < 0 || i+1 >= (map.len() as i32) || j-1 < 0 || j+1 >= (map[i as usize].len() as i32) {
            return false;
        }

        let rt = map[(i+1) as usize].chars().nth((j+1) as usize).unwrap();
        let lb = map[(i-1) as usize].chars().nth((j-1) as usize).unwrap();
        let lt = map[(i-1) as usize].chars().nth((j+1) as usize).unwrap();
        let rb = map[(i+1) as usize].chars().nth((j-1) as usize).unwrap();


        if rt == 'M' && lb == 'S' &&
        lt == 'M' && rb == 'S' ||
        rt == 'S' && lb == 'M' &&
        lt == 'S' && rb == 'M' ||
        rt == 'M' && lb == 'S' &&
        lt == 'S' && rb == 'M' ||
        rt == 'S' && lb == 'M' &&
        lt == 'M' && rb == 'S' {
            return true;
        }
    } 

    return false;
}

fn find_1(map: &Vec<&str>, i:i32, j:i32, ch:char ) -> i32 {
    let mut res = 0; 

    if map[i as usize].chars().nth(j as usize).unwrap() == ch {
        for k in -1..=1 {
            for l in -1..=1 {
                for g in 1..=4 {
                    let kg:i32 = k * g;
                    let lg:i32 = l * g;

                    if g == 4
                    {
                        res += 1;
                        break;
                    }

                    if i+kg < 0 || i+kg >= (map.len() as i32) || j+lg < 0 || j+lg >= (map[i as usize].len() as i32) {
                        break;
                    }

                    if map[(i+kg) as usize].chars().nth((j+lg) as usize).unwrap() == "XMAS".chars().nth(g as usize).unwrap(){
                        continue;
                    }
                    else {
                        break;
                    }
                }
            }
        }
    } 

    return res;
}
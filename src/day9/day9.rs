use std::fs::read_to_string;

pub fn solve() {
    let file_name: &str = "src/day9/input_prod.txt";

    let line = read_to_string(file_name).unwrap().chars().map(|x| x as i32 - 0x30).collect::<Vec<i32>>();

    let mut used: Vec<Mem> = Vec::new();
    let mut empty = Vec::new();

    let mut i = 0;
    while i < line.len() {
        used.push(Mem {value: (i as i32) / 2, capacity: line[i], used: true});

        if i < line.len() - 1 {
            empty.push(line[i + 1]);
        }
       
        i += 2;
    }

    let mut result = Vec::new();

    for i in 0..used.len() {
        let used_mem = &mut used[i];
        let empty_mem = if i == empty.len() {-1} else {empty[i]};

        for _i in 0..used_mem.capacity {
            if used_mem.used {
                result.push(used_mem.value);
            } else {
                result.push(0);
            }
           
        }

        used_mem.used = false;

        if empty_mem == -1 {
            break;
        }

        let mut k = 0;
        while k < empty_mem {
            let idx = find_capacity(empty_mem - k, &used);

            if idx.is_none() {
                for _i in k..empty_mem {
                    result.push(0);
                }
                break;
            }

            let last = used.get_mut(idx.unwrap()).unwrap();

            for _i in 0..last.capacity {
                result.push(last.value);
            }

            k += last.capacity;
            last.used = false;            
        }
    }

    let mut r: i64 = 0;

    for i in 0..result.len() {
       r += (result[i] as i64) * (i as i64);
    }

    println!("Result: {}", r);

}

fn find_capacity(target:i32, used: &Vec<Mem>) -> Option<usize> {
    for i in (0..used.len()).rev() {
        if used[i].used && used[i].capacity <= target {
            return Some(i);
        }
    }

    return None;
}

struct Mem {
    value: i32,
    capacity: i32,
    used: bool
}
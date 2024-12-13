use std::fs::read_to_string;
use std::collections::HashSet;

pub fn solve() {
    let file_name: &str = "src/day12/input.txt";

    let lines = read_to_string(file_name).unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in lines.lines() {
        map.push(line.chars().collect::<Vec<char>>());
    }

    let mut price = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != '.' {
                let mut mem: HashSet<(usize, usize)> = HashSet::new(); 
                let mut fences: HashSet<(usize, usize, char)> = HashSet::new();

                let (f, s) = get_price(&mut map, i, j, &mut mem, &mut fences);

                //println!("Before {:?}", fences);
                deduplicate(&mut fences, map.len());
                //println!("After {:?}", fences);

                price += fences.len() * s;
                
                //for k in 0..map.len() {
                //    println!("{:?}", map[k]);
                //}
                //println!("Add fence: {}={}, Add s: {}", f, fences.len(), s);
            }
        }
    }

    println!("Result: {}", price);
}

fn deduplicate(fences: &mut HashSet<(usize, usize, char)>, len: usize) {
    let fences_vec = fences.clone().into_iter().collect::<Vec<(usize, usize, char)>>();
    
    for f in fences_vec  {
        if !fences.contains(&(f.0, f.1, f.2)) {
            continue;
        }

        if f.2 == 'u' || f.2 == 'd' {
            let mut i = 1;

            loop {
                if f.1 >= i && fences.contains(&(f.0, f.1 - i, f.2)) {
                    fences.remove(&(f.0, f.1 - i, f.2));
                    i += 1;
                } else {
                    break;
                }
            }

            i = 1;

            loop {
                if f.1 + i < len && fences.contains(&(f.0, f.1 + i, f.2)) {
                    fences.remove(&(f.0, f.1 + i, f.2));
                    i += 1;
                } else {
                    break;
                }
            }
        }

        if f.2 == 'r' || f.2 == 'l' {
            let mut i = 1;

            loop {
                if f.0 >= i && fences.contains(&(f.0 - i, f.1, f.2)) {
                    fences.remove(&(f.0 - i, f.1, f.2));
                    i += 1;
                } else {
                    break;
                }
            }

            i = 1;

            loop {
                if f.0 + i < len && fences.contains(&(f.0 + i, f.1, f.2)) {
                    fences.remove(&(f.0 + i, f.1, f.2));
                    i += 1;
                } else {
                    break;
                }
            }
        }
    }
}

fn get_price(map: &mut Vec<Vec<char>>, row: usize, col: usize, mem:&mut HashSet<(usize, usize)>, fences: &mut HashSet<(usize, usize, char)>) -> (usize, usize) {
    let mut fence = 0;
    let mut square = 1;

    mem.insert((row, col));

    if row == 0 || (map[row - 1][col] != map[row][col] && !mem.contains(&(row - 1, col))) {
        fence += 1;
        fences.insert((row, col, 'u'));
    } else if !mem.contains(&(row - 1, col)) {
        let (new_fence, new_square) = get_price(map, row - 1, col, mem, fences);

        fence += new_fence;
        square += new_square;
    }

    if row == map.len() - 1 || (map[row + 1][col] != map[row][col] && !mem.contains(&(row + 1, col))) {
        fence += 1;
        fences.insert((row, col, 'd'));
    } else if !mem.contains(&(row + 1, col)) {
        let (new_fence, new_square) = get_price(map, row + 1, col, mem, fences);

        fence += new_fence;
        square += new_square;
    }

    if col == 0 || (map[row][col - 1] != map[row][col] && !mem.contains(&(row, col - 1))) {
        fence += 1;
        fences.insert((row, col, 'l'));
    } else if !mem.contains(&(row, col - 1)) {
        let (new_fence, new_square) = get_price(map, row, col - 1, mem, fences);
            
        fence += new_fence;
        square += new_square;
    }

    if col == map[row].len() - 1 || (map[row][col + 1] != map[row][col] && !mem.contains(&(row, col + 1))) {
        fence += 1;
        fences.insert((row, col, 'r'));
    } else if !mem.contains(&(row, col + 1)) {
        let (new_fence, new_square) = get_price(map, row, col + 1, mem, fences);

        fence += new_fence;
        square += new_square;
    }

    map[row][col] = '.';

    return (fence, square);
}
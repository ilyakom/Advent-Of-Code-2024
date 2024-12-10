use std::fs::read_to_string;

pub fn solve() {
    let file_name: &str = "src/day10/input.txt";

    let lines = read_to_string(file_name).unwrap();

    let mut map: Vec<Vec<usize>> = Vec::new();
    let mut heads = Vec::new();
    
    let mut j = 0;

    for line in lines.lines() {
        let heights = line.chars().map(|x| x as usize - 0x30).collect::<Vec<usize>>();

        for i in 0..heights.len() {
            if heights[i] == 0 {
                heads.push((j, i));
            }
        }

        map.push(heights);
        j += 1;
    }

    let mut result = 0;

    for head in heads {
        result += get_score(&map, head);
    }

    println!("Result: {}", result);

}

fn get_score(map: &Vec<Vec<usize>>, coord: (usize, usize)) -> usize {
    if map[coord.0 as usize][coord.1 as usize] == 9 {
        return 1;
    }

    let mut result = 0;

    if coord.0 >= 1 && map[coord.0 - 1][coord.1] == map[coord.0][coord.1] + 1 {
        result += get_score(map, (coord.0 - 1, coord.1));
    }

    if coord.0 + 1 < map.len() && map[coord.0 + 1][coord.1] == map[coord.0][coord.1] + 1 {
        result += get_score(map, (coord.0 + 1, coord.1));
    }

    if coord.1 >= 1 && map[coord.0][coord.1 - 1] == map[coord.0][coord.1] + 1 {
        result += get_score(map, (coord.0, coord.1 - 1));
    }

    if coord.1 + 1 < map[0].len() && map[coord.0][coord.1 + 1] == map[coord.0][coord.1] + 1 {
        result += get_score(map, (coord.0, coord.1 + 1));
    }

    return result;
}
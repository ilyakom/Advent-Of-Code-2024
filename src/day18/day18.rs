use std::{collections::{HashMap, HashSet, VecDeque}, fs::read_to_string};

pub fn solve() {
    let file_name: &str = "src/day18/input.txt";

    let lines = read_to_string(file_name).unwrap();
    let mut all_lines: Vec<Vec<usize>> = Vec::new(); 
    let len = 71;
    let safe_bytes = 2000;

    for line in lines.lines() {
        let pair:Vec<usize> = line.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        all_lines.push(pair);
    }

    let mut is_path_safe = false;
    let mut map: Vec<Vec<char>> = Vec::new();

    for l in safe_bytes..all_lines.len() {
        if is_path_safe {
            if map[all_lines[l][1]][all_lines[l][0]] == '.' {
                map[all_lines[l][1]][all_lines[l][0]] = '#';
                println!("Path is safe. Line: {}", l);
                continue;
            } else {
                println!("Path got hit. Line: {}", l);
                is_path_safe = false;
            }
        }

        map = build(l, &all_lines, len);
        let mut deer = Deer::new(0, 0, len);

        let path: HashSet<(usize, usize)> = HashSet::new();
        print(&deer, &path, &map);
        let (res, path) = run(&mut map, &mut deer);
        print(&deer, &path, &map);

        if res == usize::MAX {
            println!("Resut not found. Line: {}", l);
            return;
        } else {
            println!("Result: {}. iteration: {}, ", res, l);
            is_path_safe = true;
            
            for p in path.iter() {
                map[p.1][p.0] = '+';
            }
        }
    }
}

fn build(take: usize, all_lines: &Vec<Vec<usize>>, len: usize) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = Vec::new();

    for _i in 0..len {
        let mut ln = Vec::with_capacity(len);
        for _j in 0..len {
            ln.push('.');
        }
        map.push(ln);
    }

    for i in 0..=take {
        let pair = &all_lines[i];
        map[pair[1]][pair[0]] = '#';
    }

    let mut to_paint = Vec::new();

    for i in 1..len-1 {
        for j in 1..len - 1 {
            if map[i-1][j-1] == '.'
                && map[i-1][j] == '.'
                && map[i-1][j+1] == '.'
                && map[i][j-1] == '.'
                && map[i][j+1] == '.'
                && map[i+1][j-1] == '.'
                && map[i+1][j] == '.'
                && map[i+1][j+1] == '.' {
                    to_paint.push((i, j));
                }
        }
    }

    for p in to_paint {
        map[p.0][p.1] = '#';
    }

    return map;
}

fn run(map: &mut Vec<Vec<char>>, deer: &mut Deer) -> (usize, HashSet<(usize, usize)>)  {

    let mut mem: HashMap<(usize, usize), usize> = HashMap::new();
    let mut stack: VecDeque<Deer> = VecDeque::new();

    mem.insert((deer.x, deer.y), 0);
    stack.push_back(deer.clone());

    let mut highest_score = usize::MAX;
    let mut path: HashSet<(usize, usize)> = HashSet::new();

    let mut iter = 0;

    while stack.len() > 0 {

        let mut deer = stack.pop_front().unwrap();

        iter += 1;

        if deer.score > highest_score {
            continue;
        }
    
        if deer.y == map.len() - 1 && deer.x == map.len() - 1 {       
            if deer.score < highest_score {
                highest_score = deer.score;
                path.clear();
            }

            if deer.score == highest_score {
                for p in deer.tiles.iter() {
                    path.insert(*p);
                }
            } 
            
            continue;
        }

        if deer.can_turn_right(map) {
            let mut new_deer = deer.clone();
            new_deer.turn_right();
            new_deer.walk_strait();

            if mem.contains_key(&(new_deer.x, new_deer.y)) {
                let score = mem.get(&(new_deer.x, new_deer.y)).unwrap();
                if new_deer.score <= *score {
                    mem.insert((new_deer.x, new_deer.y), new_deer.score);
                    stack.push_back(new_deer);
                }
            } else {
                mem.insert((new_deer.x, new_deer.y), new_deer.score);
                stack.push_back(new_deer);
            }
        }
    
        if deer.can_turn_left(map) {
            let mut new_deer = deer.clone();
            new_deer.turn_left();
            new_deer.walk_strait();

            if mem.contains_key(&(new_deer.x, new_deer.y)) {
                let score = mem.get(&(new_deer.x, new_deer.y)).unwrap();
                if new_deer.score <= *score {
                    mem.insert((new_deer.x, new_deer.y), new_deer.score);
                    stack.push_back(new_deer);
                }
            } else {
                mem.insert((new_deer.x, new_deer.y), new_deer.score);
                stack.push_back(new_deer);
            } 
        }
    
        if deer.can_walk_strait(map) {
            deer.walk_strait();

            if mem.contains_key(&(deer.x, deer.y)) {
                let score = mem.get(&(deer.x, deer.y)).unwrap();
                if deer.score <= *score {
                    mem.insert((deer.x, deer.y), deer.score);
                    stack.push_back(deer);
                }
            } else {
                mem.insert((deer.x, deer.y), deer.score);
                stack.push_back(deer);
            } 
        }
    }

    return (highest_score, path);

}

fn print(deer: &Deer, mem: &HashSet<(usize, usize)>, map: &Vec<Vec<char>>) {
    for i in 0..map.len() {
        for j in 0..map.len() {
            if deer.x == j && deer.y == i {
                print!("{}", "D");
            } else if mem.contains(&(j, i)) {
                print!("{}", "+");
            } else {
                print!("{}", map[i][j]);
            }
        }
        println!();
    }

    println!();
}

#[derive(Debug, Clone)]
struct Deer {
    x: usize,
    y: usize,
    dir: char,
    score: usize,
    tiles: HashSet<(usize, usize)>,
    size: usize,
}

impl Deer {
    fn new(x: usize, y: usize, size:usize) -> Deer {
        let mut deer = Deer {
            x,
            y,
            dir:'r',
            score: 0,
            tiles: HashSet::new(),
            size: size
        };

        deer.tiles.insert((x, y));

        return deer;
    }

    fn walk_strait(&mut self) {
        match self.dir {
            'u' => self.y -= 1,
            'r' => self.x += 1,
            'd' => self.y += 1,
            'l' => self.x -= 1,
            _ => panic!("Invalid direction"),
        }

        self.score += 1;
        self.tiles.insert((self.x, self.y));
    }

    fn can_walk_strait(&self, map: &Vec<Vec<char>>) -> bool {
        match self.dir {
            'u' => self.y > 0 && map[self.y - 1][self.x] == '.',
            'r' => self.x + 1 < self.size && map[self.y][self.x + 1] == '.',
            'd' => self.y + 1 < self.size && map[self.y + 1][self.x] == '.',
            'l' => self.x > 0 && map[self.y][self.x - 1] == '.',
            _ => panic!("Invalid direction"),
        }
    }

    fn can_turn_right(&self, map: &Vec<Vec<char>>) -> bool {
        match self.dir {
            'u' => self.x + 1 < self.size && map[self.y][self.x + 1] == '.',
            'r' => self.y + 1 < self.size && map[self.y + 1][self.x] == '.',
            'd' => self.x > 0 && map[self.y][self.x - 1] == '.',
            'l' => self.y > 0 && map[self.y - 1][self.x] == '.',
            _ => panic!("Invalid direction"),
        }
    }

    fn turn_right(&mut self) {
        match self.dir {
            'u' => self.rotate_to('r'),
            'r' => self.rotate_to('d'),
            'd' => self.rotate_to('l'),
            'l' => self.rotate_to('u'),
            _ => panic!("Invalid direction"),
        }
    }

    fn can_turn_left(&self, map: &Vec<Vec<char>>) -> bool {
        match self.dir {
            'u' => self.x > 0 && map[self.y][self.x - 1] == '.',
            'r' => self.y > 0 && map[self.y - 1][self.x] == '.',
            'd' => self.x + 1 < self.size && map[self.y][self.x + 1] == '.',
            'l' => self.y + 1 < self.size && map[self.y + 1][self.x] == '.',
            _ => panic!("Invalid direction"),
        }
    }

    fn turn_left(&mut self) {
        match self.dir {
            'u' => self.rotate_to('l'),
            'r' => self.rotate_to('u'),
            'd' => self.rotate_to('r'),
            'l' => self.rotate_to('d'),
            _ => panic!("Invalid direction"),
        }
    }

    fn rotate_to(&mut self, new_dir: char) {
        if self.dir == new_dir {
            return;
        }

        let is_clockwise = match new_dir {
            'u' => self.dir == 'l',
            'r' => self.dir == 'u',
            'd' => self.dir == 'r',
            'l' => self.dir == 'd',
            _ => panic!("Invalid new direction")
        };

        let mut rot_num = 0;

        while self.dir != new_dir {
            rot_num += 1;
            self.rotate(is_clockwise);
        }

        if rot_num > 1 {
            println!("----------- Rotated {} times", rot_num);
        };
    }

    fn rotate(&mut self, clockwise: bool) {
        if !clockwise {
            self.dir = match self.dir {
                'u' => 'l',
                'l' => 'd',
                'd' => 'r',
                'r' => 'u',
                _ => panic!("Invalid direction"),
            }
        } else {
            self.dir = match self.dir {
                'u' => 'r',
                'r' => 'd',
                'd' => 'l',
                'l' => 'u',
                _ => panic!("Invalid direction"),
            }
        }
    }
}
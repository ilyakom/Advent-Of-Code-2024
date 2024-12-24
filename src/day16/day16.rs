use std::{collections::{HashMap, HashSet, VecDeque}, fs::read_to_string};

pub fn solve() {
    let file_name: &str = "src/day16/input.txt";

    let lines = read_to_string(file_name).unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in lines.lines() {
        let mut ln = Vec::new();
        for ch in line.chars() {
            ln.push(ch);
        }

        map.push(ln);
    }

    let mut deer = Deer::new(1, map.len() - 2 );

    run(&mut map, &mut deer);
}

fn run(map: &mut Vec<Vec<char>>, deer: &mut Deer) {

    let mut mem: HashMap<(usize, usize, char), usize> = HashMap::new();
    let mut stack: VecDeque<Deer> = VecDeque::new();

    mem.insert((deer.x, deer.y, deer.dir), 0);
    stack.push_back(deer.clone());

    let mut highest_score = usize::MAX;
    let mut path: HashSet<(usize, usize)> = HashSet::new();

    while stack.len() > 0 {
        let mut deer = stack.pop_front().unwrap();

        if deer.score > highest_score {
            continue;
        }
    
        if deer.y == 1 && deer.x == map[1].len() - 2 {       
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

            if mem.contains_key(&(new_deer.x, new_deer.y, new_deer.dir)) {
                let score = mem.get(&(new_deer.x, new_deer.y, new_deer.dir)).unwrap();
                if new_deer.score <= *score {
                    mem.insert((new_deer.x, new_deer.y, new_deer.dir), new_deer.score);
                    stack.push_back(new_deer);
                }
            } else {
                mem.insert((new_deer.x, new_deer.y, new_deer.dir), new_deer.score);
                stack.push_back(new_deer);
            }


        }
    
        if deer.can_turn_left(map) {
            let mut new_deer = deer.clone();
            new_deer.turn_left();
            new_deer.walk_strait();

            if mem.contains_key(&(new_deer.x, new_deer.y, new_deer.dir)) {
                let score = mem.get(&(new_deer.x, new_deer.y, new_deer.dir)).unwrap();
                if new_deer.score <= *score {
                    mem.insert((new_deer.x, new_deer.y, new_deer.dir), new_deer.score);
                    stack.push_back(new_deer);
                }
            } else {
                mem.insert((new_deer.x, new_deer.y, new_deer.dir), new_deer.score);
                stack.push_back(new_deer);
            } 
        }
    
        if deer.can_walk_strait(map) {
            deer.walk_strait();

            if mem.contains_key(&(deer.x, deer.y, deer.dir)) {
                let score = mem.get(&(deer.x, deer.y, deer.dir)).unwrap();
                if deer.score <= *score {
                    mem.insert((deer.x, deer.y, deer.dir), deer.score);
                    stack.push_back(deer);
                }
            } else {
                mem.insert((deer.x, deer.y, deer.dir), deer.score);
                stack.push_back(deer);
            } 
        }
    }

    println!("{}", highest_score);
    println!("{}", path.len());

}

fn print(deer: &Deer, mem: &HashMap<(usize, usize, char), usize>, map: &Vec<Vec<char>>) {
    for i in 0..15 {
        for j in 0..15 {
            if deer.x == j && deer.y == i {
                print!("{}", "D");
            } else if mem.contains_key(&(j, i, 'u')) || mem.contains_key(&(j, i, 'r')) || mem.contains_key(&(j, i, 'l')) || mem.contains_key(&(j, i, 'd')) {
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
}

impl Deer {
    fn new(x: usize, y: usize) -> Deer {
        let mut deer = Deer {
            x,
            y,
            dir:'r',
            score: 0,
            tiles: HashSet::new(),
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
            'u' => map[self.y - 1][self.x] == '.',
            'r' => map[self.y][self.x + 1] == '.',
            'd' => map[self.y + 1][self.x] == '.',
            'l' => map[self.y][self.x - 1] == '.',
            _ => panic!("Invalid direction"),
        }
    }

    fn can_turn_right(&self, map: &Vec<Vec<char>>) -> bool {
        match self.dir {
            'u' => map[self.y][self.x + 1] == '.',
            'r' => map[self.y + 1][self.x] == '.',
            'd' => map[self.y][self.x - 1] == '.',
            'l' => map[self.y - 1][self.x] == '.',
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
            'u' => map[self.y][self.x - 1] == '.',
            'r' => map[self.y - 1][self.x] == '.',
            'd' => map[self.y][self.x + 1] == '.',
            'l' => map[self.y + 1][self.x] == '.',
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

        self.score += 1000;
    }
}
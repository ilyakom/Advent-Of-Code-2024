use std::fs::read_to_string;
use std::collections::HashMap;

pub fn solve() {
    let file_name: &str = "src/day15/input.txt";

    let lines = read_to_string(file_name).unwrap();
    let mut map: HashMap<(i64, i64), Obect> = HashMap::new();

    let mut reading_map = true;
    let mut robot: Option<Obect> = None;

    let mut line_num = 0;
    for line in lines.lines() {

        if line.is_empty() {
            reading_map = false;
            continue;
        }

        if reading_map {
            let mut col_num = 0;
            for c in line.chars() {
                let tile = match c {
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    '@' => Tile::Robot,
                    _ => Tile::None
                };

                if tile == Tile::Wall {
                    let obj_1 = Obect {
                        x: col_num,
                        y: line_num,
                        tile: tile
                    };

                    let obj_2 = Obect {
                        x: col_num+1,
                        y: line_num,
                        tile: tile
                    };

                    map.insert((col_num, line_num), obj_1);
                    map.insert((col_num+1, line_num), obj_2);
                } else if tile == Tile::Box {
                    let obj_l = Obect {
                        x: col_num,
                        y: line_num,
                        tile: Tile::LBox
                    };

                    let obj_r = Obect {
                        x: col_num+1,
                        y: line_num,
                        tile: Tile::RBox
                    };

                    map.insert((col_num, line_num), obj_l);
                    map.insert((col_num+1, line_num), obj_r);
                } else if tile == Tile::Robot {
                    let obj = Obect {
                        x: col_num,
                        y: line_num,
                        tile: tile
                    };

                    robot = Some(obj);
                    map.insert((col_num, line_num), obj);
                }

                col_num += 2;
            }

            line_num += 1;
        } else {
            for c in line.chars() {
                let dir = match c {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => panic!("Invalid direction")
                };

                println!("Move: {}", c);

                robot.as_mut().unwrap().move_to(&dir, &mut map);

                //print!("\x1B[2J\x1B[1;1H");
            }
        }
    }

    for i in 0..50  {
        for j in 0..50 {
            if map.contains_key(&(j, i)) {
                let pic = match map[&(j, i)].tile {
                    Tile::Wall => '#',
                    Tile::LBox => '[',
                    Tile::RBox => ']',
                    Tile::Robot => '@',
                    _ => '.'
                };

                print!("{}", pic);
            }
            else {
                print!("{}", '.');
            }
        }
        println!()
    }

    let mut result = 0;

    for o in map.values() {
        if o.tile == Tile::LBox {
            result += o.y * 100 + o.x;
        }
    }

    println!("Result: {}", result);
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Wall,
    Box,
    LBox,
    RBox,
    Robot,
    None
}

#[derive(Debug, Clone, Copy)]
struct Obect {
    x: i64,
    y: i64,
    tile: Tile,
}

impl Move for Obect {
    fn can_move_to(&mut self, dir: &Direction, map:&mut HashMap<(i64, i64), Obect>) -> bool {
        if self.tile == Tile::Wall {
            return false;
        }

        match dir {
            Direction::Up => {
                if map.contains_key(&(self.x, self.y - 1)) {
                    let mut itm = map[&(self.x, self.y - 1)];
                    if !itm.can_move_to(dir, map) {
                        return false;
                    }
                }

                if self.tile == Tile::LBox {
                    if map.contains_key(&(self.x + 1, self.y - 1)) {
                        let mut itm = map[&(self.x + 1, self.y - 1)];
                        if !itm.can_move_to(&dir, map) {
                            return false;
                        }
                    }
                } else if self.tile == Tile::RBox {
                    if map.contains_key(&(self.x - 1, self.y - 1)) {
                        let mut itm = map[&(self.x - 1, self.y - 1)];
                        if !itm.can_move_to(&dir, map) {
                            return false;
                        }
                    }
                }

                return true;
            },
            Direction::Down => {
                if map.contains_key(&(self.x, self.y + 1)) {
                    let mut itm = map[&(self.x, self.y + 1)];
                    if !itm.can_move_to(dir, map) {
                        return false;
                    }
                }

                if self.tile == Tile::LBox {
                    if map.contains_key(&(self.x + 1, self.y + 1)) {
                        let mut itm = map[&(self.x + 1, self.y + 1)];
                        if !itm.can_move_to(&dir, map) {
                            return false;
                        }
                    }
                } else if self.tile == Tile::RBox {
                    if map.contains_key(&(self.x - 1, self.y + 1)) {
                        let mut itm = map[&(self.x - 1, self.y + 1)];
                        if !itm.can_move_to(&dir, map) {
                            return false;
                        }
                    }
                }

                return true;
            },
            Direction::Left => {
                if map.contains_key(&(self.x - 1, self.y)) {
                    let mut itm = map[&(self.x - 1, self.y)];
                    if !itm.can_move_to(dir, map) {
                        return false;
                    }
                }

                return true;
            },
            Direction::Right => {
                if map.contains_key(&(self.x + 1, self.y)) {
                    let mut itm = map[&(self.x + 1, self.y)];
                    if !itm.can_move_to(dir, map) {
                        return false;
                    }
                }

                return true;
            }
        }
    }

    fn move_to(&mut self, dir: &Direction, map: &mut HashMap<(i64, i64), Obect>) {
        if self.tile == Tile::Wall {
            return;
        }

        if !self.can_move_to(dir, map) {
            return;
        }

        match dir {
            Direction::Up => {
                if map.contains_key(&(self.x, self.y - 1)) {
                    let mut itm = map[&(self.x, self.y - 1)];
                    itm.move_to(dir, map);
                }

                if self.tile == Tile::LBox {
                    if map.contains_key(&(self.x + 1, self.y - 1)) {
                        let mut itm = map[&(self.x + 1, self.y - 1)];
                        itm.move_to(dir, map);
                    }
                } else if self.tile == Tile::RBox {
                    if map.contains_key(&(self.x - 1, self.y - 1)) {
                        let mut itm = map[&(self.x - 1, self.y - 1)];
                        itm.move_to(dir, map);
                    }
                }
                

                let mut obj = map[&(self.x, self.y)];
                map.remove(&(obj.x, obj.y));
                obj.y -= 1;
                map.insert((obj.x, obj.y), obj);

                if self.tile == Tile::LBox {
                    let mut obj = map[&(self.x + 1, self.y)];
                    map.remove(&(obj.x, obj.y));
                    obj.y -= 1;
                    map.insert((obj.x, obj.y), obj);
                } else if self.tile == Tile::RBox{
                    let mut obj = map[&(self.x - 1, self.y)];
                    map.remove(&(obj.x, obj.y));
                    obj.y -= 1;
                    map.insert((obj.x, obj.y), obj);
                }

                if self.tile == Tile::Robot {
                    self.y -= 1;
                }
            },
            Direction::Down => {
                if map.contains_key(&(self.x, self.y + 1)) {
                    let mut itm = map[&(self.x, self.y + 1)];
                    itm.move_to(dir, map);
                }

                if self.tile == Tile::LBox {
                    if map.contains_key(&(self.x + 1, self.y + 1)) {
                        let mut itm = map[&(self.x + 1, self.y + 1)];
                        itm.move_to(dir, map);
                    }
                } else if self.tile == Tile::RBox {
                    if map.contains_key(&(self.x - 1, self.y + 1)) {
                        let mut itm = map[&(self.x - 1, self.y + 1)];
                        itm.move_to(dir, map);
                    }
                }

                let mut obj = map[&(self.x, self.y)];
                map.remove(&(obj.x, obj.y));
                obj.y += 1;
                map.insert((obj.x, obj.y), obj);

                if self.tile == Tile::LBox {
                    let mut obj = map[&(self.x + 1, self.y)];
                    map.remove(&(obj.x, obj.y));
                    obj.y += 1;
                    map.insert((obj.x, obj.y), obj);
                } else if self.tile == Tile::RBox {
                    let mut obj = map[&(self.x - 1, self.y)];
                    map.remove(&(obj.x, obj.y));
                    obj.y += 1;
                    map.insert((obj.x, obj.y), obj);
                }
                
                if self.tile == Tile::Robot {
                    self.y += 1;
                }
            },
            Direction::Left => {
                if map.contains_key(&(self.x - 1, self.y)) {
                    let mut itm = map[&(self.x - 1, self.y)];
                    itm.move_to(dir, map);
                }

                let mut obj = map[&(self.x, self.y)];
                map.remove(&(obj.x, obj.y));
                obj.x -= 1;
                map.insert((obj.x, obj.y), obj);

                if self.tile == Tile::Robot {
                    self.x -= 1;
                }         
            },
            Direction::Right => {
                if map.contains_key(&(self.x + 1, self.y)) {
                    let mut itm = map[&(self.x + 1, self.y)];
                    itm.move_to(dir, map);
                }

                let mut obj = map[&(self.x, self.y)];
                map.remove(&(obj.x, obj.y));
                obj.x += 1;
                map.insert((obj.x, obj.y), obj);

                if self.tile == Tile::Robot {
                    self.x+= 1;
                }
            }
        }
    }
}

trait Move {
    fn move_to(&mut self, dir: &Direction, map:&mut HashMap<(i64, i64), Obect>);

    fn can_move_to(&mut self, dir: &Direction, map:&mut HashMap<(i64, i64), Obect>) -> bool;
}

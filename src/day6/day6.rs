use std::collections::HashSet;
use std::fs::read_to_string;

pub fn solve() {
    let file_name: &str = "src/day6/input.txt";

    let mut init_route = HashSet::new();

    let mut result = 0;

    let lines = read_to_string(file_name).unwrap();

    let mut robot_x = 0;
    let mut robot_y = 0;

    let mut robot_start_x = 0;
    let mut robot_start_y = 0;
    let robot_start_dir = '^';


    let mut robot_dir = robot_start_dir; 

    let mut is_end = false;

    let mut map = Vec::new();

    for line in lines.lines() {
        map.push(line.chars().collect::<Vec<char>>());
        
        if line.contains('^') {
            robot_start_x = line.find('^').unwrap();
            robot_start_y = map.len() - 1;
        }
    }

    robot_x = robot_start_x;
    robot_y = robot_start_y;

    while !is_end {
        match robot_dir {
            '^' =>  {
                if robot_y == 0 {
                    is_end = true;
                } else if map[robot_y - 1][robot_x] == '#' {
                    robot_dir = '>';
                } else {
                    robot_y -= 1;
                    init_route.insert((robot_x, robot_y));
                }
            },
            '>' => {
                if robot_x == map[robot_y].len() - 1 {
                    is_end = true;
                } else if map[robot_y][robot_x + 1] == '#' {
                    robot_dir = 'v';
                } else {
                    robot_x += 1;
                    init_route.insert((robot_x, robot_y));
                }
            },
            'v' => {
                if robot_y == map.len() - 1 {
                    is_end = true;
                } else if map[robot_y + 1][robot_x] == '#' {
                    robot_dir = '<';
                } else {
                    robot_y += 1;
                    init_route.insert((robot_x, robot_y));
                }
            },
            '<' => {
                if robot_x == 0 {
                    is_end = true;
                } else if map[robot_y][robot_x - 1] == '#' {
                    robot_dir = '^';
                } else {
                    robot_x -= 1;
                    init_route.insert((robot_x, robot_y));
                }
            },
            _ => println!("Unknown direction: {}", robot_dir),
        }
    }

    println!("Init route: {}", init_route.len());

    for position in init_route {
        map[position.1][position.0] = '#';

        robot_x = robot_start_x;
        robot_y = robot_start_y;
        robot_dir = robot_start_dir;
        let mut cur_route = HashSet::new();

        loop {
            match robot_dir {
                '^' =>  {
                    if robot_y == 0 {
                        break;
                    } else if map[robot_y - 1][robot_x] == '#' {
                        robot_dir = '>';
                    } else {
                        robot_y -= 1;
                        let new_position = (robot_x, robot_y, robot_dir);

                        if cur_route.contains(&new_position)
                        {
                            result += 1;
                            break;
                        }

                        cur_route.insert(new_position);
                    }
                },
                '>' => {
                    if robot_x == map[robot_y].len() - 1 {
                        break;
                    } else if map[robot_y][robot_x + 1] == '#' {
                        robot_dir = 'v';
                    } else {
                        robot_x += 1;
                        let new_position = (robot_x, robot_y, robot_dir);

                        if cur_route.contains(&new_position)
                        {
                            result += 1;
                            break;
                        }

                        cur_route.insert(new_position);
                    }
                },
                'v' => {
                    if robot_y == map.len() - 1 {
                        break;
                    } else if map[robot_y + 1][robot_x] == '#' {
                        robot_dir = '<';
                    } else {
                        robot_y += 1;
                        let new_position = (robot_x, robot_y, robot_dir);

                        if cur_route.contains(&new_position)
                        {
                            result += 1;
                            break;
                        }

                        cur_route.insert(new_position);
                    }
                },
                '<' => {
                    if robot_x == 0 {
                        break;
                    } else if map[robot_y][robot_x - 1] == '#' {
                        robot_dir = '^';
                    } else {
                        robot_x -= 1;
                        let new_position = (robot_x, robot_y, robot_dir);

                        if cur_route.contains(&new_position)
                        {
                            result += 1;
                            break;
                        }

                        cur_route.insert(new_position);
                    }
                },
                _ => println!("Unknown direction: {}", robot_dir),
            }
        }

        map[position.1][position.0] = '.';
    }

    println!("Result: {}", result);
}

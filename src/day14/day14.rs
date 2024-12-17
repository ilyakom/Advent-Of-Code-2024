use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};
use std::io;

pub fn solve() {
    let file_name: &str = "src/day14/input.txt";

    let lines = read_to_string(file_name).unwrap();
    let mut map: Vec<Robot> = Vec::new();

    let wide = 101; //11 // 101
    let tall = 103; //7  // 103

    for line in lines.lines() {

        let pv:Vec<&str> = line.split(" ").collect();
        let p = &pv[0][2..].split(",").map(|x| { x.parse::<i64>().unwrap() }).collect::<Vec<i64>>();
        let v = &pv[1][2..].split(",").map(|x| { x.parse::<i64>().unwrap() }).collect::<Vec<i64>>();

        map.push(Robot::new(p[0], p[1], v[0], v[1]));
    }

    let mut quardrant = HashMap::new();
    
    quardrant.insert((0, wide / 2 - 1, 0, tall / 2 - 1), 0);
    quardrant.insert((wide / 2 + 1, wide - 1, 0, tall / 2 - 1), 0); 
    quardrant.insert((0, wide / 2 - 1, tall / 2 + 1, tall - 1), 0);
    quardrant.insert((wide / 2 + 1, wide - 1, tall / 2 + 1, tall - 1), 0);


    //for i in 1..10000 {
        mut_fast(&mut map, wide, tall, &mut quardrant);

        let mut points = HashSet::new();

        for rob in map.iter_mut() {
            points.insert((rob.px, rob.py));
        }

        let mut stop = false;
        
        println!("Tick: {}", 0);
        for y in 0..tall {
            let mut in_a_row = 0;
            for x in 0..wide {
                if points.contains(&(x, y)) {
                    in_a_row += 1;
                    if in_a_row > 6 {
                        stop = true;
                    }
                } else {
                    in_a_row = 0;
                }
            }
        }

        //if stop {
            for y in 0..tall {
                for x in 0..wide {
                    if points.contains(&(x, y)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        //}

        if stop {
            let mut buffer = String::new();
            let stdin = io::stdin(); // We get `Stdin` here.
            stdin.read_line(&mut buffer);
        }
    //}
}

fn mut_fast(map: &mut Vec<Robot>, wide: i64, tall: i64, quardrant: &mut HashMap<(i64, i64, i64, i64), i64>) {

    let seconds = 7502;

    for rob in map.iter_mut() {
        let x_move = (rob.px + rob.vx * seconds) % wide;
        let y_move = (rob.py + rob.vy * seconds) % tall;

        if y_move < 0 {
            rob.py = tall + y_move;
        } else {
            rob.py = y_move;
        }

        if x_move < 0 {
            rob.px = wide + x_move;
        } else {
            rob.px = x_move;
        }

        //for q in quardrant.iter_mut() {
        //    if rob.px >= q.0.0 && rob.px <= q.0.1 && rob.py >= q.0.2 && rob.py <= q.0.3 {
        //        *q.1 += 1;
        //        break;
        //    }
        //}
    }

    //let mut result = 1;

    //for q in quardrant.iter() {
    //    println!("Q={:?},  Count={:?}", q.0, q.1);
    //    result *= q.1;
    //}

    //println!("Result: {}", result);

}

struct Robot {
    px: i64,
    py: i64,

    vx: i64,
    vy: i64,
}

impl Robot {
    fn new(px: i64, py: i64, vx: i64, vy: i64) -> Robot {
        Robot {
            px,
            py,
            vx,
            vy
        }
    }
}

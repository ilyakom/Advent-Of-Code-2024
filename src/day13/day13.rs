use std::{fs::read_to_string, result};
use regex::Regex;
use std::collections::HashSet;

pub fn solve() {
    let file_name: &str = "src/day13/input.txt";

    let lines = read_to_string(file_name).unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();

    let rx = Regex::new(r"\d+").unwrap();

    let mut btn_a: Button;
    let mut btn_b: Button;

    let mut atm: Automat;

    let mut result = 0;

    let mut i = 0;
    let lines = lines.lines().collect::<Vec<&str>>();
    while i < lines.len() {
        let mut line = lines[i]; 
        i += 1;

        let mut cap = rx.captures_iter(line).map(|x| { x.get(0).unwrap().as_str() }).collect::<Vec<&str>>();

        btn_a = Button::new(cap[0].parse::<usize>().unwrap(),cap[1].parse::<usize>().unwrap(), 3);

        println!("{}", line);
        println!("A: {} {}", btn_a.x, btn_a.y);

        line = lines[i];
        i += 1;

        cap = rx.captures_iter(line).map(|x| { x.get(0).unwrap().as_str() }).collect::<Vec<&str>>();
        
        btn_b = Button::new(cap[0].parse::<usize>().unwrap(), cap[1].parse::<usize>().unwrap(), 1);

        println!("{}", line);
        println!("B: {} {}", btn_b.x, btn_b.y);

        line = lines[i];
        i += 2;

        cap = rx.captures_iter(line).map(|x| { x.get(0).unwrap().as_str() }).collect::<Vec<&str>>();
        
        atm = Automat::new(
            btn_a,
            btn_b, 
            cap[0].parse::<usize>().unwrap() + 10000000000000, 
            cap[1].parse::<usize>().unwrap() + 10000000000000);

        println!("{}", line);

        println!("Prize: {} {}", atm.prize_x, atm.prize_y);

        let mut mem = HashSet::new();

        let mid_result = play_2(&mut atm, &mut mem);

        println!("Mid result: {}", mid_result);

        if (mid_result % 1.0) == 0.0 {
            result += mid_result as usize;
        }
    }

    println!("Result: {}", result);
}

fn play_1(atm: &mut Automat, mem: &mut HashSet<(usize, usize)>) -> bool {
    if atm.is_at_prize() {
        if atm.get_cur_price() < atm.lowest_price {
            atm.lowest_price = atm.get_cur_price();

            println!("Lowest price: {}", atm.lowest_price);
            return true;
        }
    }

    if mem.contains(&(atm.cur_x, atm.cur_y)) {
        return false;
    }

    let mut result_a = false;
    let mut result_b = false;

    if atm.press_btn_a() {
        result_a = play(atm, mem);

        if  !result_a {
            mem.insert((atm.cur_x, atm.cur_y));
        }

        atm.unpress_btn_a();
    }

    
    if atm.press_btn_b() {
        result_b = play(atm, mem);

        if  !result_b {
            mem.insert((atm.cur_x, atm.cur_y));
        }

        atm.unpress_btn_b();
    }

    

    return result_a || result_b;
}

fn play_2(atm: &mut Automat, mem: &mut HashSet<(usize, usize)>) -> f64 {
   let y:f64 = (atm.btn_a.x as i64 * atm.prize_y as i64 - atm.btn_a.y as i64 * atm.prize_x as i64) as f64 / (atm.btn_a.x as i64 * atm.btn_b.y as i64 - atm.btn_a.y as i64 * atm.btn_b.x as i64) as f64;
   let x = (atm.prize_x as f64 - atm.btn_b.x as f64 * y) as f64 / atm.btn_a.x as f64;

   println!("X: {} Y: {}", x, y);

   return x * atm.btn_a.p as f64 + y * atm.btn_b.p as f64;
}

struct Button {
    x: usize,
    y: usize,

    p: usize,

    btn_press_count: usize,

    limit: usize
}

impl Button {
    fn press(&mut self) -> bool {
        //if self.btn_press_count < self.limit {
            self.btn_press_count += 1;
            return true;
        //}

        //return false;
    }

    fn get_price(&self) -> usize {
        return self.p * self.btn_press_count; 
    }

    fn new(x:usize, y:usize, p:usize) -> Button {
        Button {
            x: x,
            y: y,
            p: p,
            btn_press_count: 0,
            limit: 100
        }
    }
    
}

struct Automat {
    btn_a: Button,
    btn_b: Button,

    prize_x: usize,
    prize_y: usize,

    cur_x: usize,
    cur_y: usize,

    lowest_price: usize
}

impl Automat {
    fn new(a:Button, b:Button, p_x:usize, p_y:usize) -> Automat {
        Automat {
            btn_a: a,
            btn_b: b,

            prize_x: p_x,
            prize_y: p_y,

            cur_x: 0,
            cur_y: 0,

            lowest_price: usize::MAX
        }
    }

    fn press_btn_a(&mut self) -> bool {
        if self.cur_x + self.btn_a.x > self.prize_x || self.cur_y + self.btn_a.y > self.prize_y {
            return false;
        }

        if self.btn_a.press() {
            self.cur_x += self.btn_a.x;
            self.cur_y += self.btn_a.y;

            return true;
        }

        return false;
    }

    fn press_btn_b(&mut self) -> bool {
        if self.cur_x + self.btn_b.x > self.prize_x || self.cur_y + self.btn_b.y > self.prize_y {
            return false;
        }

        if self.btn_b.press() {
            self.cur_x += self.btn_b.x;
            self.cur_y += self.btn_b.y;

            return true;
        }
        
        return false;
    }

    fn unpress_btn_a(&mut self) {
        self.cur_x -= self.btn_a.x;
        self.cur_y -= self.btn_a.y;

        self.btn_a.btn_press_count -= 1;
    }

    fn unpress_btn_b(&mut self) {
        self.cur_x -= self.btn_b.x;
        self.cur_y -= self.btn_b.y;

        self.btn_b.btn_press_count -= 1;
    }

    fn is_at_prize(&self) -> bool {
        self.cur_x == self.prize_x && self.cur_y == self.prize_y
    }

    fn get_cur_price(&self) -> usize {
        return self.btn_a.get_price() + self.btn_b.get_price();
    }
}
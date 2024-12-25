use std::{collections::{HashMap, HashSet, VecDeque}, fs::read_to_string};

pub fn solve() {
    let file_name: &str = "src/day17/input.txt";

    let lines = read_to_string(file_name).unwrap();

    let mut pr = Program::new();

    for line in lines.lines() {
        if line.starts_with("Register A:") {
            let parts: Vec<&str> = line.split(" ").collect();
            pr.a = parts[2].parse::<isize>().unwrap();
        } else if line.starts_with("Register B:") {
            let parts: Vec<&str> = line.split(" ").collect();
            pr.b = parts[2].parse::<isize>().unwrap();
        } else if line.starts_with("Register C:") {
            let parts: Vec<&str> = line.split(" ").collect();
            pr.c = parts[2].parse::<isize>().unwrap();
        } else if line.starts_with("Program:") {
            let parts: Vec<&str> = line.split(" ").collect();
            pr.program = parts[1].split(",").map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>();
        }
    }

    let mut min_diff = u64::MAX;

    // PART 2: this number got from the following alg: serch from 000 to 111 to get last number from Program. Then shift 3 bits left. Search from (for example 101000 to ...) for 2 last numbers from Program and so on. I don't want to automate it. I did it manually.
    for a in 202322348616232.. {
        pr.a = a;
        pr.reset();
        pr.execute();
        let result = pr.result.clone().into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
        let diff = 2411750314455530_u64.abs_diff(result.parse::<u64>().unwrap());

        if diff < min_diff {
            min_diff = diff;
            println!("a={}, res={}, min_diff = {}", a, result, min_diff);
        }

        if diff == 0 {
            break;
        }
    }
}


struct Program {
    a: isize,
    b: isize,
    c: isize,
    pointer: usize,
    program: Vec<isize>,
    result: Vec<isize>,
}

impl Program {
    fn new() -> Program {
        Program {
            a: 0,
            b: 0,
            c: 0,
            pointer: 0,
            program: Vec::new(),
            result: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.b = 0;
        self.c = 0;
        self.pointer = 0;
        self.result.clear();
    }

    fn execute(&mut self) {
        while self.pointer < self.program.len() {
            let opcode = self.program[self.pointer];
            let mut operand = self.program[self.pointer + 1];

            if opcode != 1 {
                operand = self.get_operand(operand);
            }

            self.get_by_opcode(opcode, operand);
        } 
    }

    fn get_operand(&self, operand: isize) -> isize {
        let result = match operand  {
            0..4 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid opcode"), 
        };

        return result;
    }

    fn get_by_opcode(&mut self, opcode: isize, operand: isize) {
        match opcode {
            0 => {
                let base: isize = 2;
                self.a = self.a / base.pow(operand as u32);
                self.pointer += 2;
            },
            1 => {
                let left = self.b;
                let right = operand;
                self.b = left ^ right;
                self.pointer += 2;
            },
            2 => {
                self.b = operand % 8;
                self.pointer += 2;
            },
            3 => {
                if self.a != 0 {
                    self.pointer = operand as usize;
                } else {
                    self.pointer += 2;
                }
            },
            4 => {
                self.b = self.b ^ self.c;
                self.pointer += 2;
            },
            5 => {
                self.result.push(operand % 8);
                self.pointer += 2;
            },
            6 => {
                let base: isize = 2;
                self.b = self.a / base.pow(operand as u32);
                self.pointer += 2;
            },
            7 => {
                let base: isize = 2;
                self.c = self.a / base.pow(operand as u32);
                self.pointer += 2;
            },
            _ => panic!("Invalid opcode"), 
        };
    }
    
}


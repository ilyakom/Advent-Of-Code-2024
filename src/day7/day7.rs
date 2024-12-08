use std::fs::read_to_string;

pub fn solve() {
    let file_name: &str = "src/day7/input.txt";

    let mut result = 0;

    let lines = read_to_string(file_name).unwrap();

    for line in lines.lines() {
        let eq = line.split(":").collect::<Vec<&str>>();

        let left = eq[0].parse::<i64>().unwrap();
        let nums = eq[1].trim().split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        if check(&nums, 0, 0, left) {

            result += left;
            //println!("{}", line);
        }

        
    }

    println!("Result: {}", result);

}

fn check(nums:&Vec<i64>, idx:usize, cur:i64, res:i64) -> bool {

    for i in idx..nums.len() {
        let mut new_cur: i64;

        if cur != 0 {
            new_cur = cur * nums[i];

            if new_cur == res {
                return true;
            }

            if new_cur < res && check(nums, i + 1, new_cur, res) {
                return true;
            }
        }

        new_cur = cur + nums[i];

        if new_cur == res {
            return true;
        }

        if new_cur < res && check(nums, i + 1, new_cur, res) {
            return true;
        }

        new_cur = (cur.to_string() + &nums[i].to_string()).parse::<i64>().unwrap();

        if new_cur == res {
            return true;
        }

        if new_cur < res && check(nums, i + 1, new_cur, res) {
            return true;
        }
    }

    return false;
}
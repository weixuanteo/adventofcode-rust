use std::fs;

pub fn solve(file_path: &String) {
    let mut total = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let t: Vec<i32> = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        // will panic if there's a non int, but its ok since the inputs should be clean from adventofcode
        // might want to properly handle this, but i will just let it panic for now
        if is_safe(&t) {
            // println!("found safe level: {:?}", t);
            total += 1
        }
    }
    println!("Total safe: {total}")
}

fn is_safe(level: &Vec<i32>) -> bool {
    if level.len() < 2 {
        return true;
    }
    // check the first two numbers to determine the order
    let is_ascending = level[0] < level[1];

    for i in 0..level.len() - 1 {
        let diff = level[i].abs_diff(level[i + 1]);
        if diff == 0 || diff > 3 {
            return false;
        }
        let diff = level[i] - level[i + 1];
        if (is_ascending && diff >= 0) || (!is_ascending && diff <= 0) {
            return false;
        }
    }
    return true;
}

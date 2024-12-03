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
        let (safe, problem_indexes) = is_safe(&t, -1);

        if safe {
            // println!("found safe level: {:?}", t);
            total += 1
        } else {
            // try removing the problematic level one by one, check if it is ok
            for i in 0..10 {
                if problem_indexes[i] == 1 {
                    let (safe, _) = is_safe(&t, i as i32);
                    if safe {
                        total += 1;
                        // println!(
                        //     "found safe level after removing {:?} at index {i}: {:?}",
                        //     t[i], t
                        // );
                        break;
                    }
                }
            }
        }
    }
    println!("Total safe with problem dampener: {total}")
}

fn is_safe(level: &Vec<i32>, ignore_index: i32) -> (bool, [i32; 10]) {
    let mut result = true;
    // we create a array space to mark those numbers that are problematic
    let mut problem_indexes: [i32; 10] = [0; 10]; // size 10 should be enough given the input

    let mut start: i32 = 0;
    let mut end: i32 = (level.len() - 1) as i32;
    if ignore_index == start {
        start += 1;
    }
    if ignore_index == end {
        end -= 1;
    }

    let mut is_ascending = true;
    let mut is_ascending_count = 0;
    let mut is_descending_count = 0;
    for i in start..end {
        let mut n1 = i;
        let mut n2 = i + 1;
        if n1 as i32 == ignore_index {
            n1 -= 1
        }
        if n2 as i32 == ignore_index {
            n2 += 1
        }
        if level[n1 as usize] > level[n2 as usize] {
            is_descending_count += 1;
        } else if level[n1 as usize] < level[n2 as usize] {
            is_ascending_count += 1;
        }
    }
    if is_descending_count > is_ascending_count {
        is_ascending = false;
    }

    for i in start..end {
        let mut n1 = i;
        let mut n2 = i + 1;
        if n1 as i32 == ignore_index {
            n1 -= 1
        }
        if n2 as i32 == ignore_index {
            n2 += 1
        }

        let abs_diff = level[n1 as usize].abs_diff(level[n2 as usize]);
        let diff = level[n1 as usize] - level[n2 as usize];

        if abs_diff == 0
            || abs_diff > 3
            || (is_ascending && diff >= 0)
            || (!is_ascending && diff <= 0)
        {
            result = false;
            if i + 1 == (level.len() - 1) as i32 && problem_indexes[n1 as usize] == 0 {
                // we are checking the last pair
                // if the first number is marked as ok, only mark the last number as problematic
                problem_indexes[n2 as usize] = 1;
            } else {
                // mark both as problematic
                problem_indexes[n1 as usize] = 1;
                problem_indexes[n2 as usize] = 1;
            }
        } else {
            problem_indexes[n1 as usize] = 0;
            problem_indexes[n2 as usize] = 0;
        }
    }

    return (result, problem_indexes);
}

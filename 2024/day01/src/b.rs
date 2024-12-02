use std::{collections::HashMap, fs};

pub fn solve(file_path: &String) {
    let mut a: Vec<i32> = Vec::new();
    let mut b: HashMap<i32, i32> = HashMap::new();

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let t: Vec<&str> = line.split(' ').filter(|s| !s.is_empty()).collect();
        assert!(t.len() == 2);
        let n1: i32 = t[0].to_string().parse().expect("why is this not a number");
        let n2: i32 = t[1].to_string().parse().expect("why is this not a number");
        a.push(n1);
        match b.get(&n2) {
            Some(&number) => b.insert(n2, number + 1),
            _ => b.insert(n2, 1),
        };
    }

    // sort both vectors in ascending order
    a.sort();

    // println!("{:?}", a);
    // println!("{:?}", b);

    let mut total = 0;

    for i in 0..a.len() {
        let count: i32 = match b.get(&a[i]) {
            Some(&number) => number,
            None => continue,
        };
        total += a[i] * count;
    }

    println!("Similarity score: {total}")
}

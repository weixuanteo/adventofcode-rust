use std::fs;

pub fn solve(file_path: &String) {
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let t: Vec<&str> = line.split(' ').filter(|s| !s.is_empty()).collect();
        assert!(t.len() == 2);
        let n1 = t[0].to_string().parse().expect("why is this not a number");
        let n2 = t[1].to_string().parse().expect("why is this not a number");
        a.push(n1);
        b.push(n2);
    }

    // sort both vectors in ascending order
    a.sort();
    b.sort();

    // println!("{:?}", a);
    // println!("{:?}", b);

    let mut total = 0;

    for i in 0..a.len() {
        let diff = a[i].abs_diff(b[i]);
        total += diff
    }

    println!("Total distance: {total}")
}

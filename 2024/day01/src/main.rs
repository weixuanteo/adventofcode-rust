use std::env;
mod a;
mod b;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("not enough arguments!")
    }

    let file_path = &args[1];

    println!("reading input from {file_path}");

    a::solve(file_path);
    b::solve(file_path);
}

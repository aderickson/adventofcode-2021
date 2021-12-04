mod file;
mod solutions;

use crate::file::FileReader;

use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path;

    if args.len() < 2 {
        path = "input.txt";
    } else if args.len() > 2 {
        panic!("Expected 0 or 1 arguments");
    } else {
        path = args[1].as_str();
    }

    println!("Reading {}", path);

    // let file = fs::read_to_string(path).unwrap();

    let file = FileReader::new(path);

    let start = Instant::now();
    let result = solutions::part_one(file);
    let duration = start.elapsed();

    println!("Part 1: {} ({} μs)", result, duration.as_micros());

    let file = FileReader::new(path);

    let start = Instant::now();
    let result = solutions::part_two(file);
    let duration = start.elapsed();

    println!("Part 2: {} ({} μs)", result, duration.as_micros());
}

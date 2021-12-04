use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::time::Instant;

mod solutions;

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

    let file = File::open(path).expect("Could not open input file");
    let lines = io::BufReader::new(file).lines();
    let start = Instant::now();
    let result = solutions::part_one(lines);
    let duration = start.elapsed();

    println!("Part 1: {} ({} μs)", result, duration.as_micros());

    let file = File::open(path).expect("Could not open input file");
    let lines = io::BufReader::new(file).lines();
    let start = Instant::now();
    let result = solutions::part_two(lines);
    let duration = start.elapsed();

    println!("Part 2: {} ({} μs)", result, duration.as_micros());
}

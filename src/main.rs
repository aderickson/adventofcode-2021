use std::str::Split;
use std::{env, fs};
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

    let file = fs::read_to_string(path).unwrap();

    let lines = file.split('\n');
    let start = Instant::now();
    let result = part_one(lines);
    let duration = start.elapsed();

    println!("Part 1: {} ({} μs)", result, duration.as_micros());


    let lines = file.split('\n');
    let start = Instant::now();
    let result = part_two(lines);
    let duration = start.elapsed();

    println!("Part 2: {} ({} μs)", result, duration.as_micros());
}

fn part_one(lines : Split<char>) -> u32 {
    let measurements : Vec<u32> = lines.map(|line|
        line.parse().unwrap()
    ).collect();

    let mut num_increasing = 0;

    for index in 0..(measurements.len() - 1) {
        if measurements[index] < measurements[index + 1] {
            num_increasing += 1;
        }
    }

    return num_increasing;
}

fn part_two(lines : Split<char>) -> u32 {
    let measurements : Vec<u32> = lines.map(|line|
        line.parse().unwrap()
    ).collect();

    let mut num_increasing = 0;

    for index in 0..(measurements.len() - 3) {
        if measurements[index] < measurements[index + 3] {
            num_increasing += 1;
        }
    }
    
    return num_increasing
}
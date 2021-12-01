use std::env;
use std::time::Instant;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read, Seek};

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

    let file_handle = File::open(path).expect("Unable to open file");
    let mut reader = BufReader::new(file_handle);

    let line_iter = reader.by_ref().lines();
    let start = Instant::now();
    let result = part_one(line_iter);
    let duration = start.elapsed();

    println!("Part 1: {} ({} μs)", result, duration.as_micros());

    reader.rewind().unwrap();

    let line_iter = reader.by_ref().lines();
    let start = Instant::now();
    let result = part_two(line_iter);
    let duration = start.elapsed();

    println!("Part 2: {} ({} μs)", result, duration.as_micros());
}

fn part_one(lines : Lines<&mut BufReader<File>>) -> u32 {
    let measurements = lines.map(|line|
        line.unwrap().parse().unwrap()
    );

    let mut num_increasing = 0;
    let mut previous = 0;

    for (index, measurement) in measurements.enumerate() {
        if index == 0 {
            previous = measurement;
        } else if previous < measurement {
            previous = measurement;
            num_increasing += 1;
        } else {
            previous = measurement;
        }
    }

    return num_increasing;
}

fn part_two(lines : Lines<&mut BufReader<File>>) -> u32 {
    let measurements : Vec<u32> = lines.map(|line|
        line.unwrap().parse().unwrap()
    ).collect();

    let mut num_increasing = 0;

    for index in 0..(measurements.len() - 3) {
        if measurements[index] < measurements[index + 3] {
            num_increasing += 1;
        }
    }
    
    return num_increasing
}
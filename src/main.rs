use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

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
    let lines = BufReader::new(file_handle).lines();

    let measurements : Vec<u32> = lines.map(|line|
        line.unwrap().parse().unwrap()
    ).collect();

    let mut num_increasing = 0;

    for index in 0..(measurements.len() - 3) {
        if measurements[index] < measurements[index + 3] {
            num_increasing += 1;
        }
    }

    println!("{}", num_increasing);
}

use std::{env, fs};

#[derive(Debug)]
struct PasswordLine<'a> {
    character : char,
    position_one : usize,
    position_two : usize,
    string : &'a str
}

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

    let text_string = fs::read_to_string(path).unwrap();
    let lines = text_string.split('\n');

    let measurements : Vec<u32> = lines.map(|line|
        line.parse().unwrap()
    ).collect();

    let mut num_increasing = 0;

    for index in 0..(measurements.len() - 3) {
        if measurements[index] < measurements[index + 3] {
            num_increasing += 1;
        }
    }

    println!("{}", num_increasing);
}

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

    let measurements = lines.map(|line|
        line.parse::<u32>().unwrap()
    );

    let mut num_increasing = 0;
    let mut previous = 0;

    for (index, measurement) in measurements.enumerate() {
        if index == 0 {
            previous = measurement;
            println!("{}: {}", index, measurement);
            continue;
        }

        if previous < measurement {
            previous = measurement;
            num_increasing += 1;
            println!("{}: {}, increasing", index, measurement);
        } else {
            previous = measurement;
            println!("{}: {}, not increasing", index, measurement);
        }
    }

    println!("{}", num_increasing);
}

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    assert!(args.len() == 2, "Expected exactly 1 argument");
    println!("Reading {}", args[1]);

    let path = args[1].as_str();
    let file_handle = File::open(path).expect("Unable to open file");
    let read_stream = BufReader::new(file_handle);

    let lines = read_stream.lines();

    for (index, line) in lines.enumerate() {
        let text = line.expect("Read error");
        println!("{: >4} > {}", index, text);
    }
}

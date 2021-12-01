use std::{env, fs};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use regex::Regex;

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

    let regex = Regex::new(
    r#"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)"#
    ).unwrap();

    let passwords = regex.captures_iter(text_string.as_str()).map(|captures|
        PasswordLine {
            position_one: captures.get(1).unwrap().as_str().parse().unwrap(),
            position_two: captures.get(2).unwrap().as_str().parse().unwrap(),
            character: captures.get(3).unwrap().as_str().chars().next().unwrap(),
            string: captures.get(4).unwrap().as_str()
        }
    );

    let mut num_valid = 0;

    for password in passwords {
        let chars : Vec<char> = password.string.chars().collect();

        if chars[password.position_one - 1] == password.character || chars[password.position_two - 1] == password.character {
            if chars[password.position_one - 1] != password.character || chars[password.position_two - 1] != password.character{
                num_valid += 1;
            }
        }
    }

    println!("{}", num_valid);
}

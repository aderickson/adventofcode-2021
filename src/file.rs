use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::iter::Peekable;

pub struct FileReader {
    lines: Peekable<Lines<BufReader<File>>>,
    last: String,
    count: usize,
}

#[allow(dead_code)]
impl FileReader {
    pub fn new(path: &str) -> Self {
        let lines = if let Ok(file) = File::open(path) {
            io::BufReader::with_capacity(1024000, file).lines().peekable()
        } else {
            println!("ERR: Could not open input file, {}", path);
            panic!();
        };

        FileReader {
            lines,
            last: String::new(),
            count: 0,
        }
    }

    pub fn get_width_next(&mut self) -> usize {
        self.lines
            .peek()
            .unwrap()
            .as_ref()
            .map(|s| s.len())
            .unwrap()
    }

    pub fn get_width_prev(&self) -> usize {
        self.last.len()
    }

    pub fn get_linenum_next(&self) -> usize {
        self.count + 1
    }

    pub fn get_linenum_prev(&self) -> usize {
        self.count
    }
}

impl Iterator for FileReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next();

        if let Some(res) = line {
            if let Ok(string) = res {
                self.last = string;
                self.count += 1;

                return Some(self.last.clone());
            };
        };

        return None;
    }
}

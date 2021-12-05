use std::{collections::HashMap, str::FromStr};

use crate::file::FileReader;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: u32,
    y: u32
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords : Vec<&str> = s.split(',').collect();

        assert!(coords.len() == 2, "Wrong number of coordinates");

        let point = Point {
            x: coords[0].parse().unwrap(),
            y: coords[1].parse().unwrap()
        };

        return Ok(point)
    }
}

struct Line {
    start: Point,
    end: Point,

    last: Option<Point>
}

impl Line {
    pub fn new(start : Point, end : Point) -> Self {
        return Line {
            start,
            end,
            last: None
        };
    }

    fn get_step(goal : u32, current : u32) -> i32 {
        if goal == current {
            0
        } else if goal > current {
            1
        } else {
            -1
        }
    }
}

impl Iterator for Line {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last.is_none() {
            self.last = Some(self.start);
            return self.last
        }

        if self.last.unwrap() == self.end {
            return None
        }

        let last = self.last.unwrap();

        let step_x = Line::get_step(self.end.x, last.x);
        let step_y = Line::get_step(self.end.y, last.y);

        self.last = Some(Point {
            x: (last.x as i32 + step_x) as u32,
            y: (last.y as i32 + step_y) as u32
        });

        return self.last
    }
}

pub fn part_one(lines : FileReader) -> u32 {
    let lines = lines.map(|line| {
        let points : Vec<&str> = line.as_str().split(" -> ").collect();
        
        let start = points[0].parse().unwrap();
        let end = points[1].parse().unwrap();

        return Line::new(start, end);
    }).filter(|line| 
        line.start.x == line.end.x || line.start.y == line.end.y
    );

    let mut intersections = HashMap::new();

    for line in lines {
        for point in line {
            if let Some(current) = intersections.get_mut(&point) {
                let value = *current + 1;
                *current = value;
            } else {
                intersections.insert(point, 1);
            }
        }
    }

    let count = intersections.iter().filter(|(_, value)| {
        **value >= 2
    }).count();

    return count as u32;
}

pub fn part_two(lines : FileReader) -> u32 {
    let lines = lines.map(|line| {
        let points : Vec<&str> = line.as_str().split(" -> ").collect();
        
        let start = points[0].parse().unwrap();
        let end = points[1].parse().unwrap();

        return Line::new(start, end);
    });

    let mut intersections = HashMap::new();

    for line in lines {
        for point in line {
            if let Some(current) = intersections.get_mut(&point) {
                let value = *current + 1;
                *current = value;
            } else {
                intersections.insert(point, 1);
            }
        }
    }

    let count = intersections.iter().filter(|(_, value)| {
        **value >= 2
    }).count();

    return count as u32;
}
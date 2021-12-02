use std::str::FromStr;

enum Direction {
    Up,
    Down,
    Forward,
    Backward
}

#[derive(Debug)]
struct AocError {}

impl FromStr for Direction {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            "backward" => Ok(Direction::Backward),
            _ => return Err(AocError{})
        }
    }
}

struct NavStep {
    direction: Direction,
    length: i32
}

impl FromStr for NavStep {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words : Vec<&str> = s.split(' ').collect();

        if words.len() != 2 {
            return Err(AocError {})
        }

        let direction = words[0].parse::<Direction>()?;
        let length = words[1].parse::<i32>();

        if length.is_err() {
            return Err(AocError {});
        }

        Ok(NavStep {
            direction,
            length: length.unwrap()
        })
    }
}

pub fn part_one<'a>(lines : impl Iterator<Item = &'a str>) -> i32 {
    let steps = lines.map(|line| {
        line.parse::<NavStep>().unwrap()
    });

    let mut total_lateral : i32 = 0;
    let mut total_vertical : i32 = 0;
    
    for step in steps {
        match step.direction {
            Direction::Forward => total_lateral += step.length,
            Direction::Backward => total_lateral -= step.length,
            Direction::Down => total_vertical += step.length,
            Direction::Up => total_vertical -= step.length,
        }
    }

    return total_lateral * total_vertical;
}

pub fn part_two<'a>(lines : impl Iterator<Item = &'a str>) -> i32 {
    let steps = lines.map(|line| {
        line.parse::<NavStep>().unwrap()
    });

    let mut total_lateral : i32 = 0;
    let mut total_vertical : i32 = 0;
    let mut aim : i32 = 0;
    
    for step in steps {
        match step.direction {
            Direction::Forward => {
                total_lateral += step.length;
                total_vertical += aim * step.length
            },
            Direction::Backward => {
                total_lateral -= step.length;
                total_vertical -= aim * step.length
            }
            Direction::Down => aim += step.length,
            Direction::Up => aim -= step.length,
        }
    }

    return total_lateral * total_vertical;
}
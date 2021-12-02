enum Direction {
    Up,
    Down,
    Forward,
    Backward
}

struct NavStep {
    direction: Direction,
    length: i32
}

pub fn part_one<'a>(lines : impl Iterator<Item = &'a str>) -> i32 {
    let steps = lines.map(|line| {
        let words : Vec<&str> = line.split(' ').collect();

        assert!(words.len() == 2, "Line did not have two words");

        let direction = match words[0] {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            "backward" => Direction::Backward,
            _ => panic!("Unknown direction")
        };

        let length = words[1].parse().unwrap();

        NavStep {
            direction,
            length
        }
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

pub fn part_two<'a>(lines : impl Iterator<Item = &'a str>) -> u32 {
    panic!("Not implemented yet");
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
use std::str::Split;

pub fn part_one(lines : Split<char>) -> u32 {
  let measurements : Vec<u32> = lines.map(|line|
      line.parse().unwrap()
  ).collect();

  let mut num_increasing = 0;

  for index in 0..(measurements.len() - 1) {
      if measurements[index] < measurements[index + 1] {
          num_increasing += 1;
      }
  }

  return num_increasing;
}

pub fn part_two(lines : Split<char>) -> u32 {
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
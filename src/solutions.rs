use crate::file::FileReader;

use std::collections::VecDeque;

pub fn part_one(lines : FileReader) -> u32 {
    let lines : Vec<String> = lines.collect();
    assert!(lines.len() == 1);
    let starting_numbers = lines[0].split(',').map(|s| {
        s.parse::<usize>().expect("Unable to parse number")
    });

    let mut counts : VecDeque<u32> = VecDeque::new();

    for _ in 0..=8 {
        counts.push_back(0)
    }

    for num in starting_numbers {
        let prev = counts.get_mut(num).unwrap();
        *prev = *prev + 1;
    }

    #[cfg(debug_assertions)] let total = counts.iter().map(|p| {*p}).reduce(|a, e| { a + e }).unwrap();
    #[cfg(debug_assertions)] println!("Start: {}, {}, {}, {}, {}, {}, {}, {} - ({})", counts[0], counts[1], counts[2], counts[3], counts[4], counts[5], counts[6], counts[7], total);

    for day in 1..=80 {
        let giving_birth = counts.pop_front().unwrap();

        let six = counts.get_mut(6).unwrap();
        *six = *six + giving_birth;

        counts.push_back(giving_birth);

        #[cfg(debug_assertions)] let total = counts.iter().map(|p| {*p}).reduce(|a, e| { a + e }).unwrap();
        #[cfg(debug_assertions)] println!("Day {}: {}, {}, {}, {}, {}, {}, {}, {} - ({})", day, counts[0], counts[1], counts[2], counts[3], counts[4], counts[5], counts[6], counts[7], total);
    }
    
    return counts.iter().map(|ptr| {*ptr}).reduce(|acc, elem| {acc + elem}).unwrap();
}

pub fn part_two(lines : FileReader) -> u64 {
    let lines : Vec<String> = lines.collect();
    assert!(lines.len() == 1);
    let starting_numbers = lines[0].split(',').map(|s| {
        s.parse::<usize>().expect("Unable to parse number")
    });

    let mut counts = VecDeque::new();

    for _ in 0..=8 {
        counts.push_back(0)
    }

    for num in starting_numbers {
        let prev = counts.get_mut(num).unwrap();
        *prev = *prev + 1;
    }

    #[cfg(debug_assertions)] let total = counts.iter().map(|p| {*p}).reduce(|a, e| { a + e }).unwrap();
    #[cfg(debug_assertions)] println!("Start: {}, {}, {}, {}, {}, {}, {}, {} - ({})", counts[0], counts[1], counts[2], counts[3], counts[4], counts[5], counts[6], counts[7], total);

    for day in 1..=256 {
        let giving_birth = counts.pop_front().unwrap();

        let six = counts.get_mut(6).unwrap();
        *six = *six + giving_birth;

        counts.push_back(giving_birth);

        #[cfg(debug_assertions)] let total = counts.iter().map(|p| {*p}).reduce(|a, e| { a + e }).unwrap();
        #[cfg(debug_assertions)] println!("Day {}: {}, {}, {}, {}, {}, {}, {}, {} - ({})", day, counts[0], counts[1], counts[2], counts[3], counts[4], counts[5], counts[6], counts[7], total);
    }
    
    return counts.iter().map(|ptr| {*ptr}).reduce(|acc, elem| {acc + elem}).unwrap();

}
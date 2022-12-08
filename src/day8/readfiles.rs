use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read() -> Vec<Vec<i32>> {
    let file = File::open("./dataset/day8.txt").unwrap();
    let reader = BufReader::new(file);

    let vec: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split("")
                .filter(|s| !s.is_empty())
                .map(|s| (s.parse::<i32>().unwrap() + 1))
                .collect()
        })
        .collect();

    vec
}

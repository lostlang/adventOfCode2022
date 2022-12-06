use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read() -> String {
    let file = File::open("./dataset/day6.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    return reader.lines().next().unwrap().unwrap();
}

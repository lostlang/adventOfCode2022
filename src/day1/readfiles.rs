use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read() -> Vec<Option<u32>> {
    let file = File::open("./dataset/day1.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut vec = <Vec<Option<u32>>>::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let number: Option<u32> = line.parse().ok();
        vec.push(number);
    }

    vec
}

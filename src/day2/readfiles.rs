use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read() -> Vec<String> {
    let file = File::open("./dataset/day2.txt").expect("Unable to open file");

    let mut vec = <Vec<String>>::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        vec.push(line.unwrap());
    }

    vec
}

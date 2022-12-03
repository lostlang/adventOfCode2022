use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read() -> Vec<String> {
    let file = File::open("./dataset/day2.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut vec = <Vec<String>>::new();

    for line in reader.lines() {
        vec.push(line.unwrap());
    }

    vec
}

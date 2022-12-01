use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn read() -> Vec<Option<u32>> {
    let file = File::open("./dataset/day1.txt")
        .expect("Unable to open file");
    let mut vec = <Vec<Option<u32>>>::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line
            .expect("Unable to read line");

        let number: Option<u32> = line.parse().ok();

        vec.push(number);
    };

    vec
}


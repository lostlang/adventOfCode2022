use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read() -> Vec<(String, String)> {
    let file = File::open("./dataset/day3.txt").expect("Unable to open file");

    let mut vec = <Vec<(String, String)>>::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let len = line.len();

                vec.push((line[0..len / 2].to_string(), line[len / 2..len].to_string()));
            }
            _ => (),
        }
    }

    vec
}

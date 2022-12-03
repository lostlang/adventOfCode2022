use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read() -> Vec<(String, String)> {
    let file = File::open("./dataset/day3.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut vec = <Vec<(String, String)>>::new();

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

pub fn read2() -> Vec<(String, String, String)> {
    let file = File::open("./dataset/day3.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut vec = <Vec<String>>::new();
    let mut vec2 = <Vec<(String, String, String)>>::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                vec.push(line);
            }
            _ => (),
        }
    }

    for i in 0..vec.len() / 3 {
        vec2.push((
            vec[i * 3].to_string(),
            vec[i * 3 + 1].to_string(),
            vec[i * 3 + 2].to_string(),
        ));
    }

    vec2
}

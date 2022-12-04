use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read() -> Vec<((i32, i32), (i32, i32))> {
    let file = File::open("./dataset/day4.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut vec = <Vec<((i32, i32), (i32, i32))>>::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let split: Vec<&str> = line.split(",").collect();
                let part1: Vec<i32> = split[0]
                    .split("-")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();
                let part2: Vec<i32> = split[1]
                    .split("-")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect();

                vec.push(((part1[0], part1[1]), (part2[0], part2[1])));
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    vec
}

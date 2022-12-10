use std::fs::File;
use std::io::{BufRead, BufReader};

pub enum Command {
    Noop,
    Add(i32),
}

pub fn read() -> Vec<Command> {
    let file = File::open("./dataset/day10.txt").unwrap();
    let reader = BufReader::new(file);

    let mut vec: Vec<Command> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains("noop") {
            vec.push(Command::Noop);
        } else if line.contains("add") {
            vec.push(Command::Add(
                line.split(" ").last().unwrap().parse().unwrap(),
            ));
        }
    }

    vec
}

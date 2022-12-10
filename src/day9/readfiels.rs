use std::fs::File;
use std::io::{BufRead, BufReader};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn read() -> Vec<(Direction, i32)> {
    let file = File::open("./dataset/day9.txt").unwrap();
    let reader = BufReader::new(file);

    let vec: Vec<(Direction, i32)> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.split(" ").collect::<Vec<&str>>();

            let direction = match line[0] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Unknown direction"),
            };

            let distance = line[1].parse::<i32>().unwrap();
            (direction, distance)
        })
        .collect();

    vec
}

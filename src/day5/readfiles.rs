use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read() -> (Vec<Vec<String>>, Vec<Vec<i32>>) {
    let file = File::open("./dataset/day5.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut columns: Vec<Vec<String>> = Vec::new();
    for _ in 0..9 {
        columns.push(Vec::new());
    }

    let mut comands: Vec<Vec<i32>> = Vec::new();
    let mut line_count = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line_count < 8 {
            let mut index = 0;
            let table_line = table_line_generator(&line);

            for v in table_line {
                if !v.is_empty() {
                    columns[index].push(v.to_string());
                }

                index += 1;
            }
        }

        if line_count > 9 {
            comands.push(
                line.split(" ")
                    .filter(|s| s.parse::<i32>().is_ok())
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect(),
            )
        }

        line_count += 1;
    }

    return (columns, comands);
}

fn table_line_generator(string: &str) -> Vec<&str> {
    let mut line: Vec<&str> = string.split(" ").collect();

    let mut index = 0;
    while index < line.len() {
        if line[index].is_empty() {
            line.remove(index);
            line.remove(index);
            line.remove(index);
            index += 1;
        } else {
            index += 1;
        }
    }

    for i in 0..line.len() {
        if !line[i].is_empty() {
            line[i] = line[i].get(1..2).unwrap();
        }
    }

    line
}

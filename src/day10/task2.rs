use crate::day10;
use crate::day10::readfiles::Command;

pub fn main() {
    let values = day10::readfiles::read();
    let mut lines: Vec<Vec<String>> = Vec::new();
    let mut line: Vec<String> = Vec::new();
    let mut machine = vec![0, 1, 2];

    for value in values {
        match value {
            Command::Noop => {
                add_to_line(&mut line, &machine);
                add_to_lines(&mut lines, &mut line);
            }
            Command::Add(x) => {
                add_to_line(&mut line, &machine);
                add_to_lines(&mut lines, &mut line);

                add_to_line(&mut line, &machine);
                add_to_lines(&mut lines, &mut line);

                machine = validate_add(machine, x);
            }
        }
    }

    for line in lines {
        for item in line {
            print!("{} ", item);
        }
        println!();
    }
    println!("Task 2: {}", 0);
}

fn validate_add(machine: Vec<i32>, value: i32) -> Vec<i32> {
    let mut new_machine: Vec<i32> = Vec::new();

    for val in machine {
        let mut val = val + value;

        // if val < 0 {
        //     val += 40;
        // }
        // if val > 39 {
        //     val -= 40;
        // }

        new_machine.push(val);
    }

    new_machine
}

fn add_to_line(line: &mut Vec<String>, machine: &Vec<i32>) {
    let ticks = line.len() as i32;
    if machine.contains(&ticks) {
        line.push("#".to_string());
    } else {
        line.push(".".to_string());
    }
}

fn add_to_lines(lines: &mut Vec<Vec<String>>, line: &mut Vec<String>) {
    if line.len() == 40 {
        lines.push(line.clone());
        line.clear();
    }
}

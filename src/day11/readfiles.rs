use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<i32>,
    pub operation: Operation,
    pub operands: (Operand, Operand),
    pub test: i32,
    pub move_to: (i32, i32),
}

impl Monkey {
    fn new(lines: Vec<String>) -> Monkey {
        let items = lines[1]
            .split_whitespace()
            .map(|x| x.replace(",", ""))
            .filter(|x| x.parse::<i32>().is_ok())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let operation = lines[2].split_whitespace().collect::<Vec<&str>>();
        let operands = (
            Operand::new(operation[operation.len() - 1].to_string()),
            Operand::new(operation[operation.len() - 3].to_string()),
        );
        let operation = Operation::new(operation[operation.len() - 2].to_string());

        let test = get_int(&lines[3]);

        let move_to = (get_int(&lines[4]), get_int(&lines[5]));

        Monkey {
            items,
            operation,
            operands,
            test,
            move_to,
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn new(op: String) -> Operation {
        match op.as_str() {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!("Unknown operation"),
        }
    }

    pub fn get_value(&self, items: (i32, i32)) -> i32 {
        match self {
            Operation::Add => items.0 + items.1,
            Operation::Multiply => items.0 * items.1,
        }
    }
}

#[derive(Debug)]
pub enum Operand {
    SelfOperand,
    Value(i32),
}

impl Operand {
    fn new(operand: String) -> Operand {
        match operand.parse::<i32>() {
            Ok(value) => Operand::Value(value),
            Err(_) => Operand::SelfOperand,
        }
    }

    pub fn get_value(&self, val: i32) -> i32 {
        match self {
            Operand::SelfOperand => val,
            Operand::Value(value) => *value,
        }
    }
}

pub fn read() -> Vec<Monkey> {
    let file = File::open("./dataset/day11.txt").unwrap();
    let reader = BufReader::new(file);

    let mut vec: Vec<Monkey> = Vec::new();
    let mut monkey_container: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            vec.push(Monkey::new(monkey_container));
            monkey_container = Vec::new();
        } else {
            monkey_container.push(line);
        }
    }

    vec
}

fn get_int(s: &String) -> i32 {
    s.split_whitespace()
        .filter(|x| x.parse::<i32>().is_ok())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()[0]
}

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<i128>,
    pub operation: Operation,
    pub operands: (Operand, Operand),
    pub test: i128,
    pub move_to: (i128, i128),
}

impl Monkey {
    fn new(lines: Vec<String>) -> Monkey {
        let items = lines[1]
            .split_whitespace()
            .map(|x| x.replace(",", ""))
            .filter(|x| x.parse::<i128>().is_ok())
            .map(|x| x.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();

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

    pub fn get_value(&self, items: (i128, i128)) -> i128 {
        match self {
            Operation::Add => items.0 + items.1,
            Operation::Multiply => {
                // println!("{} * {}", items.0, items.1);
                items.0 * items.1
            }
        }
    }
}

#[derive(Debug)]
pub enum Operand {
    SelfOperand,
    Value(i128),
}

impl Operand {
    fn new(operand: String) -> Operand {
        match operand.parse::<i128>() {
            Ok(value) => Operand::Value(value),
            Err(_) => Operand::SelfOperand,
        }
    }

    pub fn get_value(&self, val: i128) -> i128 {
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

fn get_int(s: &String) -> i128 {
    s.split_whitespace()
        .filter(|x| x.parse::<i128>().is_ok())
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>()[0]
}

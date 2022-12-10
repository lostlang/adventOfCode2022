use crate::day10;
use crate::day10::readfiles::Command;

pub fn main() {
    let values = day10::readfiles::read();
    let mut sum = 0;
    let mut ticks = 1;
    let mut machine = 1;
    let need_ticks = vec![20, 60, 100, 140, 180, 220];

    for value in values {
        match value {
            Command::Noop => {
                ticks += 1;
                sum += machine_tick(ticks, machine, &need_ticks);
            }
            Command::Add(x) => {
                ticks += 1;
                sum += machine_tick(ticks, machine, &need_ticks);

                machine += x;
                ticks += 1;
                sum += machine_tick(ticks, machine, &need_ticks);
            }
        }
    }

    println!("Task 1: {}", sum);
}

fn machine_tick(ticks: i32, machine: i32, need_ticks: &Vec<i32>) -> i32 {
    if need_ticks.contains(&ticks) {
        return ticks * machine;
    }

    return 0;
}

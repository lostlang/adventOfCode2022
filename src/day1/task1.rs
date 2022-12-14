use crate::day1;

pub fn main() {
    let values = day1::readfiles::read();

    let mut max = 0;
    let mut current = 0;

    for v in values {
        match v {
            Some(x) => {
                current += x;
            }
            None => {
                if current > max {
                    max = current;
                };

                current = 0;
            }
        }
    }

    println!("Task 1: {}", max);
}

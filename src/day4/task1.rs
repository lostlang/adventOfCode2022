use crate::day4;
use std::cmp::Ordering;

pub fn main() {
    let values = day4::readfiles::read();

    let mut count = 0;

    for v in values {
        let start = v.0 .0.cmp(&v.1 .0);
        let end = v.0 .1.cmp(&v.1 .1);

        match (start, end) {
            (Ordering::Equal, Ordering::Equal) => count += 1,
            (Ordering::Equal, Ordering::Less) => count += 1,
            (Ordering::Less, Ordering::Equal) => count += 1,
            (Ordering::Equal, Ordering::Greater) => count += 1,
            (Ordering::Greater, Ordering::Equal) => count += 1,
            (Ordering::Less, Ordering::Greater) => count += 1,
            (Ordering::Greater, Ordering::Less) => count += 1,
            _ => (),
        }
    }

    println!("Task 1: {}", count);
}

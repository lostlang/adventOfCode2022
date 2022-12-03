use crate::day3;
use std::collections::HashMap;

pub fn main() {
    let values = day3::readfiles::read();

    let mut count = 0;

    'word: for (left, right) in values {
        let mut map = HashMap::new();

        for c in left.chars() {
            map.insert(c, 0);
        }

        for c in right.chars() {
            match map.get(&c) {
                Some(_) => {
                    let b = convert_to_int(c);
                    count += b;
                    continue 'word;
                }
                None => (),
            }
        }
    }

    println!("Task 1: {}", count);
}

pub fn convert_to_int(c: char) -> i16 {
    let mut b = c as i16 - 'A' as i16 + 1;

    if b > 32 {
        b -= 32;
    } else {
        b += 26;
    };

    b
}

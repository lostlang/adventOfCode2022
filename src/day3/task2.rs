use crate::day3;
use std::collections::HashMap;

pub fn main() {
    let values = day3::readfiles::read2();

    let mut count = 0;

    'word: for (left, middle, right) in values {
        let mut map = HashMap::new();
        let mut map2 = HashMap::new();

        for c in left.chars() {
            map.insert(c, 0);
        }

        for c in middle.chars() {
            map2.insert(c, 0);
        }

        for c in right.chars() {
            match map.get(&c) {
                Some(_) => match map2.get(&c) {
                    Some(_) => {
                        let b = day3::task1::convert_to_int(c);
                        count += b;
                        continue 'word;
                    }
                    None => (),
                },
                None => (),
            }
        }
    }

    println!("Task 2: {}", count);
}

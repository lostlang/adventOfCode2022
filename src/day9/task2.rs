use crate::day9;
use crate::day9::task1::{move_head, move_tail};
use std::collections::HashSet;

pub fn main() {
    let values = day9::readfiels::read();
    let mut head = (0, 0);
    let mut vecTail: Vec<(i32, i32)> = Vec::new();
    for _ in 0..9 {
        vecTail.push((0, 0));
    }
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    set.insert(head);

    for (direction, distance) in values {
        for _ in 0..distance {
            head = move_head(head, &direction);
            let mut prev = head;
            for i in 0..9 {
                vecTail[i] = move_tail(vecTail[i], prev);
                prev = vecTail[i];
            }
            set.insert(vecTail[8]);
        }
    }

    println!("Task 2: {}", set.len());
}

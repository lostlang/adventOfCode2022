use crate::day9;
use std::cmp;
use std::collections::HashSet;

pub fn main() {
    let values = day9::readfiels::read();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    set.insert(head);

    for (direction, distance) in values {
        for _ in 0..distance {
            head = move_head(head, &direction);
            tail = move_tail(tail, head);
            set.insert(tail);
        }
    }

    println!("Task 1: {}", set.len());
}

fn distance(head: (i32, i32), tail: (i32, i32)) -> i32 {
    let x = (head.0 - tail.0).abs();
    let y = (head.1 - tail.1).abs();

    cmp::max(x, y)
}

pub fn move_head(head: (i32, i32), direction: &day9::readfiels::Direction) -> (i32, i32) {
    match direction {
        day9::readfiels::Direction::Up => (head.0, head.1 + 1),
        day9::readfiels::Direction::Down => (head.0, head.1 - 1),
        day9::readfiels::Direction::Left => (head.0 - 1, head.1),
        day9::readfiels::Direction::Right => (head.0 + 1, head.1),
    }
}

pub fn move_tail(tail: (i32, i32), head: (i32, i32)) -> (i32, i32) {
    let distance = distance(head, tail);

    if distance > 1 {
        let x = head.0 - tail.0;
        let y = head.1 - tail.1;

        match (x, y) {
            (2, 2) => return (tail.0 + 1, tail.1 + 1),
            (2, -2) => return (tail.0 + 1, tail.1 - 1),
            (-2, 2) => return (tail.0 - 1, tail.1 + 1),
            (-2, -2) => return (tail.0 - 1, tail.1 - 1),
            (2, _) => return (tail.0 + 1, head.1),
            (-2, _) => return (tail.0 - 1, head.1),
            (_, 2) => return (head.0, tail.1 + 1),
            (_, -2) => return (head.0, tail.1 - 1),
            _ => panic!("Unknown direction"),
        }
    }

    return tail;
}

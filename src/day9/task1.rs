use crate::day9;
use std::cmp;
use std::collections::HashSet;

pub fn main() {
    println!("Task 1");
    let values = day9::readfiels::read();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    set.insert(head);

    for (direction, distance) in values {
        for i in 0..distance {
            move_head(&mut head, &direction);
            move_tail(&mut tail, &head, &mut set);
        }
    }

    println!("Task:1 {}", set.len());
}

fn distance(head: (i32, i32), tail: (i32, i32)) -> i32 {
    let x = (head.0 - tail.0).abs();
    let y = (head.1 - tail.1).abs();

    cmp::max(x, y)
}

fn move_head(head: &mut (i32, i32), direction: &day9::readfiels::Direction) {
    match direction {
        day9::readfiels::Direction::Up => head.1 += 1,
        day9::readfiels::Direction::Down => head.1 -= 1,
        day9::readfiels::Direction::Left => head.0 -= 1,
        day9::readfiels::Direction::Right => head.0 += 1,
    }
}

fn move_tail(tail: &mut (i32, i32), head: &(i32, i32), set: &mut HashSet<(i32, i32)>) {
    let distance = distance(*head, *tail);

    if distance > 1 {
        if head.0 - tail.0 > 1 {
            tail.0 += 1;
            tail.1 = head.1;
        } else if head.0 - tail.0 < -1 {
            tail.0 -= 1;
            tail.1 = head.1;
        } else if head.1 - tail.1 > 1 {
            tail.0 = head.0;
            tail.1 += 1;
        } else if head.1 - tail.1 < -1 {
            tail.0 = head.0;
            tail.1 -= 1;
        }
    }

    set.insert(*tail);
}

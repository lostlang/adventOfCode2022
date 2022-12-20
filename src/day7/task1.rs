use crate::day7;
use crate::day7::classes::Directory;
use std::cell::RefCell;
use std::rc::Rc;

pub fn main() {
    let values = day7::readfiles::read();
    values.borrow_mut().size_update();
    let size = recursive_size(Rc::clone(&values), 0, 100_000);

    println!("Task 1: {}", size);
}

fn recursive_size(dir: Rc<RefCell<Directory>>, start: u128, end: u128) -> u128 {
    let mut out = 0;
    let dir_size = dir.borrow().size;

    if dir_size > start && dir_size < end {
        out += dir_size;
    }

    if dir.borrow().dirs.len() > 0 {
        for d in &dir.borrow().dirs {
            out += recursive_size(Rc::clone(d), start, end);
        }
    }

    out
}

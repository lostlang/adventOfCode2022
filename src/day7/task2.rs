use crate::day7;
use crate::day7::classes::Directory;
use std::cell::RefCell;
use std::rc::Rc;

pub fn main() {
    let values = day7::readfiles::read();
    values.borrow_mut().size_update();
    let size: u128 = 30_000_000 - (70_000_000 - values.borrow().size);
    let deleted = recursive_min(Rc::clone(&values), size);
    println!("Task 2: {}", deleted);
}

pub fn recursive_min(dir: Rc<RefCell<Directory>>, need: u128) -> u128 {
    let mut out = 0;
    if dir.borrow().size > need {
        out = dir.borrow().size;
    }

    for d in &dir.borrow().dirs {
        let temp = recursive_min(Rc::clone(d), need);

        if temp > 0 && temp < out {
            out = temp;
        }
    }

    out
}

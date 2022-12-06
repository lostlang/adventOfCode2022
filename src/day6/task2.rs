use crate::day6;

pub fn main() {
    let values = day6::readfiles::read();

    let mut i = 0;

    while i < values.len() - 13 {
        let word = values.get(i..i + 14).unwrap();
        let sh = day6::task1::shift(word);

        if sh > -1 {
            i += (sh + 1) as usize;
        } else {
            break;
        }
    }

    println!("Task 1: {}", i + 14);
}
